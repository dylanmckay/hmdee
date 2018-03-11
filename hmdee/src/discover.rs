use {Context, Headset};
use psvr;

/// Gets an iterator over all connected headsets.
pub fn headsets(context: &Context) -> ::std::vec::IntoIter<Headset> {
    let mut headsets = Vec::new();

    for psvr in psvr::iter(context.hidapi()).unwrap() {
        let psvr = psvr.unwrap();
        headsets.push(Headset::Psvr(psvr.into()));
    }

    headsets.into_iter()
}

