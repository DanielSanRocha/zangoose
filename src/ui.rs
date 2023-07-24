use std::sync::mpsc;

use druid::widget::{Controller, Label, Flex, Switch, Slider, LensWrap};
use druid::{Widget, WidgetExt, Env, UpdateCtx};

use crate::models::{self, AppData};

struct SliderUpdateCallback {
    tx: mpsc::Sender<f32>,
}

impl Controller<f64, Slider> for SliderUpdateCallback {
    fn update(&mut self, child: &mut Slider, ctx: &mut UpdateCtx<'_,'_>, old_data: &f64, data: &f64, env: &Env) {
        if *old_data != *data {
            let _ = self.tx.send(*data as f32);
        }

        child.update(ctx, old_data, data, env)
    }
}

struct SwitchUpdateCallback {
    tx: mpsc::Sender<bool>
}

impl Controller<bool, Switch> for SwitchUpdateCallback {
    fn update(&mut self, child: &mut Switch, ctx: &mut UpdateCtx, old_data: &bool, data: &bool, env: &Env) {
        if *old_data != *data {
            let _ = self.tx.send(*data);
        }

        child.update(ctx, old_data, data, env)
    }
}

pub fn ui_builder(tx_volume: mpsc::Sender<f32>, tx_denoise: mpsc::Sender<bool>, tx_drive: mpsc::Sender<f32>, tx_frac: mpsc::Sender<f32>) -> impl Widget<models::AppData> {
    let denoise_label = Label::new("Denoise").align_left();
    let denoise_switch = LensWrap::new(Switch::new().controller(SwitchUpdateCallback {tx: tx_denoise}), AppData::denoise).align_right();

    let frac_label = Label::new("Frac").align_left();
    let frac_slider = LensWrap::new(Slider::new().controller(SliderUpdateCallback {tx: tx_frac}), AppData::frac)
        .disabled_if(|data: &AppData, _| !data.denoise);

    let denoise_row = Flex::row()
        .with_child(denoise_label)
        .with_child(denoise_switch)
        .with_child(frac_label)
        .with_child(frac_slider)
        .align_left().padding(5.0);

    let drive_label = Label::new("Drive").align_left();
    let drive_slider = LensWrap::new(Slider::new().controller(SliderUpdateCallback {tx: tx_drive}), AppData::drive).align_right();
    let drive_row = Flex::row().with_child(drive_label).with_child(drive_slider).padding(5.0).align_left();

    let volume_label = Label::new("Volume").align_left();
    let volume_slider = LensWrap::new(Slider::new().controller(SliderUpdateCallback {tx: tx_volume}), AppData::volume).align_right();
    let volume_row = Flex::row().with_child(volume_label).with_child(volume_slider).padding(5.0).align_left();

    Flex::column().with_child(volume_row).with_child(drive_row).with_child(denoise_row).padding(5.0)
}
