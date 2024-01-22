use std::sync::{Arc, RwLock};

use wasi_common::pipe::WritePipe;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

use super::{
    task::{Task, TaskResult},
    TaskExecutor,
};

#[derive(Clone, Default)]
pub struct WasmTaskExecutor {
    engine: Engine,
}

impl std::fmt::Debug for WasmTaskExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmTaskExecutor").finish()
    }
}

impl WasmTaskExecutor {
    pub fn new() -> Self {
        WasmTaskExecutor {
            engine: Engine::default(),
        }
    }
}

impl TaskExecutor for WasmTaskExecutor {
    fn exec(&mut self, task: Task) -> anyhow::Result<TaskResult> {
        let module_name = &task.id.to_string();

        let mut linker = Linker::new(&self.engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        let stdout_buf = vec![];
        let stdout_mutex = Arc::new(RwLock::new(stdout_buf));
        let stdout = WritePipe::from_shared(stdout_mutex.clone());

        let wasi = WasiCtxBuilder::new().stdout(Box::new(stdout)).build();
        let mut store = Store::new(&self.engine, wasi);

        let module = Module::new(&self.engine, &task.module)?;
        // let module = Module::from_file(&engine, module_name)?;
        linker.module(&mut store, module_name, &module)?;

        let instance = linker.instantiate(&mut store, &module)?;
        let instance_main = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
        instance_main.call(&mut store, ())?;

        let mut buffer = Vec::new();
        stdout_mutex
            .try_read()
            .map_err(|err| anyhow::Error::msg(err.to_string()))?
            .iter()
            .for_each(|b| buffer.push(*b));

        let s = String::from_utf8(buffer)?;

        let task_result = TaskResult {
            stdout: s,
            stderr: "".to_string(),
            execution_time: 0,
        };

        Ok(task_result)
    }
}
