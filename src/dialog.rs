use mould::prelude::*;
use permission::HasPermission;
use nfd::{self, Response, DialogType};

pub enum DialogPermission {
    CanOpenSingle,
    CanSaveSingle,
    CanOpenMultiple,
}

pub struct DialogService { }

impl DialogService {

    pub fn new() -> Self {
        DialogService { }
    }

}

impl<T> Service<T> for DialogService where T: HasPermission<DialogPermission> {
    fn route(&self, request: &Request) -> Box<Worker<T>> {
        if request.action == "show-dialog" {
            Box::new(DialogWorker::new())
        } else {
            let msg = format!("Unknown action '{}' for dialog service!", request.action);
            Box::new(RejectWorker::new(msg))
        }
    }
}

struct DialogWorker {
    path: Option<String>,
    filter: Option<String>,
    dialog_type: DialogType,
}

impl DialogWorker {
    fn new() -> Self {
        DialogWorker { path: None, filter: None, dialog_type: DialogType::SingleFile }
    }
}

impl<T> Worker<T> for DialogWorker where T: HasPermission<DialogPermission> {

    fn prepare(&mut self, context: &mut T, mut request: Request) -> worker::Result<Shortcut> {
        self.path = request.extract("path");
        self.filter = request.extract("filter");
        let mode: Option<String> = request.extract("mode");
        let res = match mode.as_ref().map(String::as_ref) {
            Some("open") | None => Ok(DialogType::SingleFile),
            Some("multiple") => Ok(DialogType::MultipleFiles),
            Some("save") => Ok(DialogType::SaveFile),
            Some(mode) => Err(worker::Error::Reject(format!("Unsupported mode {}", mode))),
        };
        let dt = try!(res);
        self.dialog_type = dt;
        if context.has_permission(&DialogPermission::CanOpenSingle) {
            Ok(Shortcut::Tuned)
        } else {
            Err(worker::Error::reject("You haven't permissions!"))
        }
    }

    fn realize(&mut self, _: &mut T, _: Option<Request>) -> worker::Result<Realize> {
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
        if vec.len() > 0 {
            Ok(Realize::OneItemAndDone(mould_object!{"files" => vec}))
        } else {
            Err(worker::Error::reject("Dialog was canceled!"))
        }
    }

}
