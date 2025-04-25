pub use self::decode_queue::{DecodeQueueModel, Decoded};
pub use self::front_end::FrontEndModel;
pub use self::future_file::FutureFileModel;
pub use self::load_queue::LoadQueueModel;
pub use self::reg_file::RegFileModel;
pub use self::rob::RobModel;
pub use self::schedulers::SchedulersModel;

use cpu_sim::components::diagnostics::cpu::CpuSnapshot;

mod decode_queue;
mod front_end;
mod future_file;
mod load_queue;
mod reg_file;
mod rob;
mod schedulers;
pub mod streams;

pub struct CpuModel {
    front_end: FrontEndModel,
    decode_queue: DecodeQueueModel,
    rob: RobModel,
    schedulers: SchedulersModel,
    reg_file: RegFileModel,
    future_file: FutureFileModel,
    load_queue: LoadQueueModel,
}

impl CpuModel {
    pub fn front_end(&self) -> &FrontEndModel {
        &self.front_end
    }

    pub fn decode_queue(&self) -> &DecodeQueueModel {
        &self.decode_queue
    }

    pub fn rob(&self) -> &RobModel {
        &self.rob
    }

    pub fn schedulers(&self) -> &SchedulersModel {
        &self.schedulers
    }

    pub fn reg_file(&self) -> &RegFileModel {
        &self.reg_file
    }

    pub fn future_file(&self) -> &FutureFileModel {
        &self.future_file
    }

    pub fn load_queue(&self) -> &LoadQueueModel {
        &self.load_queue
    }
}

impl From<CpuSnapshot> for CpuModel {
    fn from(snapshot: CpuSnapshot) -> Self {
        let CpuSnapshot {
            pc,
            zb_predictor: _,
            zbp_regs,
            if_regs,
            branch_predictor: _,
            bp_regs,
            decode_width,
            decode_queue,
            rob,
            schedulers,
            reg_file,
            future_file,
            load_queue,
        } = snapshot;

        let front_end = FrontEndModel::new(pc, zbp_regs, if_regs, bp_regs, decode_width);
        let decode_queue = DecodeQueueModel::new(decode_queue);
        let rob = RobModel::new(rob);
        let schedulers = SchedulersModel::new(schedulers);
        let reg_file = RegFileModel::new(reg_file);
        let future_file = FutureFileModel::new(future_file);
        let load_queue = LoadQueueModel::new(load_queue);

        Self {
            front_end,
            decode_queue,
            rob,
            schedulers,
            reg_file,
            future_file,
            load_queue,
        }
    }
}
