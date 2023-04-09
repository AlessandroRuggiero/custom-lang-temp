struct TransformationSpace {
    pub transformations_clusters:Vec<TreansformationsCluster>
}

struct TreansformationsCluster {
    pub transformation: Vec<Transformation>
}

enum Transformation {
    LANGUAGET(LangiageTransformation)
}

trait TransformationExecutable {
    fn execute (&self);
}

struct LangiageTransformation {

}

impl TransformationExecutable for LangiageTransformation {
    fn execute (&self) {}
}

impl Transformation {
    fn execute (&self) {
        match self {
            Transformation::LANGUAGET(t) => {
                return t.execute();
            }
        };
    }
}