use mould::prelude::*;
use nfd::{self, Response, DialogType};

pub struct DialogHandler { }

impl DialogHandler {

    pub fn new() -> Self {
        DialogHandler { }
    }

}

impl<CTX> Handler<CTX> for DialogHandler {
    fn build(&self, mut request: Request) -> Box<Worker<CTX>> {
        if request.action == "show-dialog" {
            Box::new(DialogWorker {
                path: request.extract("path"),
                filter: request.extract("filter"),
                mode: request.extract("mode"),
                dialog_type: DialogType::SingleFile,
            })
        } else {
            let msg = format!("Unknown action '{}' for dialog service!", request.action);
            Box::new(RejectWorker::new(msg))
        }
    }
}

struct DialogWorker {
    path: Option<String>,
    filter: Option<String>,
    mode: Option<String>,
    dialog_type: DialogType,
}

impl<CTX> Worker<CTX> for DialogWorker {

    fn shortcut(&mut self, _: &mut CTX) -> WorkerResult<Shortcut> {
        let res = match self.mode.as_ref().map(String::as_ref) {
            Some("open") | None => Ok(DialogType::SingleFile),
            Some("multiple") => Ok(DialogType::MultipleFiles),
            Some("save") => Ok(DialogType::SaveFile),
            Some(mode) => Err(WorkerError::Reject(format!("Unsupported mode {}", mode))),
        };
        let dt = try!(res);
        self.dialog_type = dt;
        Ok(Shortcut::Tuned)
    }

    fn realize(&mut self, _: &mut CTX, _: Option<Request>) -> WorkerResult<Realize> {
        let res = try!(nfd::open_dialog(
                self.filter.as_ref().map(String::as_ref),
                self.path.as_ref().map(String::as_ref),
                self.dialog_type));
        let mut vec: Vec<String> = Vec::new();
        match res {
            Response::Okay(file) => vec.push(file),
            Response::OkayMultiple(files) => vec.extend(files),
            Response::Cancel => (), // Leave vec empty
        }
        Ok(Realize::OneItemAndDone(mould_object!{"files" => vec}))
    }

}
