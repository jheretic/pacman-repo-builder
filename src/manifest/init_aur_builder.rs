use super::{
    ArchCollectionWrapper, AurCollectionWrapper, BorrowedArchCollection, BorrowedAurCollection,
    BorrowedContainer, BorrowedFailedBuildRecord, BorrowedGnupgHome, BorrowedGpgKey,
    BorrowedPackager, BorrowedPacman, BorrowedRepository, ContainerWrapper,
    FailedBuildRecordWrapper, GlobalSettings, GnupgHomeWrapper, GpgKeyWrapper, OwnedArchCollection,
    OwnedAurCollection, OwnedContainer, OwnedFailedBuildRecord, OwnedGnupgHome, OwnedGpgKey,
    OwnedPackager, OwnedPacman, OwnedRepository, PackagerWrapper, PacmanWrapper, RepositoryWrapper,
    Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::ErrorKind, path::Path};

pub const INIT_AUR_BUILDER: &str = "init-aur-builder.yaml";

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct InitAurBuilder<
    Repository,
    Container,
    FailedBuildRecord,
    ArchCollection,
    Pacman,
    Packager,
    GnupgHome,
    GpgKey,
    AurCollection,
> where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
    GnupgHome: GnupgHomeWrapper,
    GpgKey: GpgKeyWrapper,
    AurCollection: AurCollectionWrapper,
{
    pub global_settings: GlobalSettings<
        Repository,
        Container,
        FailedBuildRecord,
        ArchCollection,
        Pacman,
        Packager,
        GnupgHome,
        GpgKey,
    >,
    pub aur_package_names: AurCollection,
}

pub type OwnedInitAurBuilder = InitAurBuilder<
    OwnedRepository,
    OwnedContainer,
    OwnedFailedBuildRecord,
    OwnedArchCollection,
    OwnedPacman,
    OwnedPackager,
    OwnedGnupgHome,
    OwnedGpgKey,
    OwnedAurCollection,
>;

pub type BorrowedInitAurBuilder<'a> = InitAurBuilder<
    BorrowedRepository<'a>,
    BorrowedContainer<'a>,
    BorrowedFailedBuildRecord<'a>,
    BorrowedArchCollection<'a>,
    BorrowedPacman<'a>,
    BorrowedPackager<'a>,
    BorrowedGnupgHome<'a>,
    BorrowedGpgKey<'a>,
    BorrowedAurCollection<'a>,
>;

impl<
        Repository,
        Container,
        FailedBuildRecord,
        ArchCollection,
        Pacman,
        Packager,
        GnupgHome,
        GpgKey,
        AurCollection,
    >
    InitAurBuilder<
        Repository,
        Container,
        FailedBuildRecord,
        ArchCollection,
        Pacman,
        Packager,
        GnupgHome,
        GpgKey,
        AurCollection,
    >
where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
    GnupgHome: GnupgHomeWrapper,
    GpgKey: GpgKeyWrapper,
    AurCollection: AurCollectionWrapper,
{
    pub fn with_global_settings(
        mut self,
        global_settings: GlobalSettings<
            Repository,
            Container,
            FailedBuildRecord,
            ArchCollection,
            Pacman,
            Packager,
            GnupgHome,
            GpgKey,
        >,
    ) -> Self {
        self.global_settings = global_settings;
        self
    }
}

impl OwnedInitAurBuilder {
    pub fn from_env() -> Result<Self, String> {
        InitAurBuilder::from_file(INIT_AUR_BUILDER.as_ref())
    }

    pub fn from_file(file: &Path) -> Result<Self, String> {
        match File::open(file) {
            Ok(content) => content
                .pipe(serde_yaml::from_reader::<_, OwnedInitAurBuilder>)
                .map_err(|error| {
                    format!("cannot deserialize {:?} as InitAurBuilder: {}", file, error)
                })?
                .pipe(Ok),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(InitAurBuilder::default()),
                _ => Err(format!("cannot open {:?} as a file: {}", file, error)),
            },
        }
    }

    pub fn with_package(mut self, package_name: String) -> Self {
        self.aur_package_names.inner_mut().push(package_name);
        self
    }
}
