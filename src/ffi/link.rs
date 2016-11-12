#[cfg(feature = "msvc12")]
#[cfg(target_family = "windows")]
#[link(name = "assimp-vc120-mt", kind = "static")]
extern {}

#[cfg(feature = "msvc14")]
#[cfg(target_family = "windows")]
#[link(name = "assimp-vc140-mt", kind = "static")]
extern {}

#[cfg(not(target_family = "windows"))]
#[link(name = "assimp", kind = "static")]
extern {}

#[link(name = "zlibstatic", kind = "static")]
extern {}