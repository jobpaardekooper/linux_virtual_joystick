use std::fmt::Debug;

use std::sync::mpsc;

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisType, AttributeSet, EventType, InputEvent, Key, UinputAbsSetup,
};

const SLIDER_AXES: [(AbsoluteAxisType, &str); 4] = [
    (AbsoluteAxisType::ABS_X, "Roll (Left X)"),
    (AbsoluteAxisType::ABS_Y, "Pitch (Left Y)"),
    (AbsoluteAxisType::ABS_RZ, "Yaw (Right Z)"),
    (AbsoluteAxisType::ABS_THROTTLE, "Throttle"),
];

const BUTTONS: [(Key, &str); 1] = [
    (Key::BTN_TRIGGER, "Panic"),
];

pub type AnalogAxis = Control<i8>;
pub type Button = Control<bool>;

pub type EventCode = u16;

pub fn build_uninput() -> anyhow::Result<(Box<[AnalogAxis]>, Box<[Button]>)> {
    let mut device = VirtualDeviceBuilder::new()?.name("Linux Virtual Joystick");

    let (event_sender, event_recv) = mpsc::channel();

    let abs_setup = AbsInfo::new(0, -100, 100, 0, 0, 1);
    let mut axes = Vec::with_capacity(SLIDER_AXES.len());
    for (axis, name) in SLIDER_AXES {
        let axis = UinputAbsSetup::new(axis, abs_setup);
        device = device.with_absolute_axis(&axis)?;
        axes.push(AnalogAxis::new(axis.code(), event_sender.clone(), name))
    }

    let mut buttons = Vec::with_capacity(SLIDER_AXES.len());
    let mut keys = AttributeSet::<Key>::new();
    for (button, name) in BUTTONS {
        keys.insert(button);
        buttons.push(Button::new(button.code(), event_sender.clone(), name))
    }
    let device = device.with_keys(&keys)?;

    let device = device.build()?;

    std::thread::spawn(|| device_thread(device, event_recv));

    Ok((axes.into_boxed_slice(), buttons.into_boxed_slice()))
}

type EventValue = i32;

#[derive(Debug)]
pub struct Control<T: Default + PartialEq + Clone + ControllerValue + Debug> {
    event_code: EventCode,
    old_value: T,
    pub new_value: T,
    event_sender: mpsc::Sender<InputEvent>,
    name: &'static str,
}

impl<T: Default + PartialEq + ControllerValue + Clone + Debug> Control<T> {
    pub fn new(
        event_code: EventCode,
        event_sender: mpsc::Sender<InputEvent>,
        name: &'static str,
    ) -> Self {
        Self {
            event_code,
            old_value: T::default(),
            new_value: T::default(),
            event_sender,
            name,
        }
    }

    pub fn new_value(&mut self) {
        if self.new_value == self.old_value {
            return;
        }
        let control_value = InputEvent::new(
            T::controller_type(),
            self.event_code,
            self.new_value.controller_value(),
        );
        self.event_sender.send(control_value).unwrap();
        self.old_value = self.new_value.clone();
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

pub trait ControllerValue {
    fn controller_type() -> EventType;
    fn controller_value(&self) -> EventValue;
}

impl ControllerValue for i8 {
    fn controller_type() -> EventType {
        EventType::ABSOLUTE
    }
    fn controller_value(&self) -> EventValue {
        i32::from(*self)
    }
}

impl ControllerValue for bool {
    fn controller_type() -> EventType {
        EventType::KEY
    }

    fn controller_value(&self) -> EventValue {
        i32::from(*self)
    }
}

// This could also be done using async rust, but that seemed to be overkill for this project.
fn device_thread(mut device: VirtualDevice, events: mpsc::Receiver<InputEvent>) {
    loop {
        let Ok(new_event) = events.recv() else {
            //Sender dropped, so app is being closed.
            break;
        };
        device.emit(&[new_event]).unwrap()
    }
}
