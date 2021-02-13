use crate::{Context, Error, Headset};
use psvr;

/// Gets an iterator over all connected headsets.
pub fn headsets(context: &Context) -> Result<::std::vec::IntoIter<Headset>, Error> {
    let mut headsets = Vec::new();

    for psvr in psvr::iter(context.hidapi())? {
        let psvr = psvr?;
        headsets.push(Headset::Psvr(psvr.into()));
    }

    Ok(headsets.into_iter())
}

