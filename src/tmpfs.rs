use async_std::path::Path;
use async_std::io;
use std::process::Command;


pub async fn fstab(sys:&Path) {
    //sudo mount -t tmpfs -o user,size=100M tmpfs /pwa/mio/
}


pub async fn mount(path:&Path) -> io::Result<()> {
    let str = path.to_str().unwrap();
    Command::new("mount").arg("-t").arg("tmpfs").arg("-t").arg("-o").arg("user,size=100M").arg("tmpfs").arg(path).spawn()?;
    Ok(())
}

pub async fn umount(path:&Path) -> io::Result<()>{
    Command::new("umount").spawn()?;
    Ok(())
}


