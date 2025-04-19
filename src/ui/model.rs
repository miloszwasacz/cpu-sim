pub use self::front_end::FrontEndModel;
pub use self::load_queue::LoadQueueModel;
pub use self::reg_file::RegFileModel;
pub use self::reg_stat::RegStatModel;
pub use self::rob::RobModel;
pub use self::schedulers::SchedulersModel;

use cpu_sim::components::diagnostics::cpu::CpuSnapshot;

mod front_end;
mod load_queue;
mod reg_file;
mod reg_stat;
mod rob;
mod schedulers;

pub struct CpuModel {
    front_end: FrontEndModel,
    rob: RobModel,
    schedulers: SchedulersModel,
    reg_file: RegFileModel,
    reg_stat: RegStatModel,
    load_queue: LoadQueueModel,
}

impl CpuModel {
    pub fn front_end(&self) -> &FrontEndModel {
        &self.front_end
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

    pub fn reg_stat(&self) -> &RegStatModel {
        &self.reg_stat
    }

    pub fn load_queue(&self) -> &LoadQueueModel {
        &self.load_queue
    }
}

impl From<CpuSnapshot> for CpuModel {
    fn from(snapshot: CpuSnapshot) -> Self {
        let CpuSnapshot {
            pc,
            if_id_regs,
            id_is_regs,
            rob,
            schedulers,
            reg_file,
            reg_stat,
            load_queue,
        } = snapshot;

        let front_end = FrontEndModel::new(pc, if_id_regs, id_is_regs);
        let rob = RobModel::new(rob);
        let schedulers = SchedulersModel::new(schedulers);
        let reg_file = RegFileModel::new(reg_file);
        let reg_stat = RegStatModel::new(reg_stat);
        let load_queue = LoadQueueModel::new(load_queue);

        Self {
            front_end,
            rob,
            schedulers,
            reg_file,
            reg_stat,
            load_queue,
        }
    }
}
