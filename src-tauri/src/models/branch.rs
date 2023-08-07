use git2::Oid;

pub struct Branch {
    pub target: Oid,
    pub merge_target: Option<Oid>,
    pub source_branch: Option<usize>,
    pub target_branch: Option<usize>,
    pub name: String,
    // what is this for?
    pub persistence: u8,
    pub is_remote: bool,
    pub is_merged: bool,
    pub is_tag: bool,
    pub svg_props: BranchSvgProps,
    pub range: (Option<usize>, Option<usize>),
}

pub struct BranchSvgProps {
    pub order_group: usize,
    pub target_order_group: Option<usize>,
    pub source_order_group: Option<usize>,
    pub colour: String,
    pub column: Option<usize>,
    pub lines: Vec<Line>,
    pub points: Vec<Point>,
}

#[derive(Default)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

#[derive(Default)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub commit_id: Option<String>,
    pub is_commit: bool,
    pub is_merge: bool,
}

impl Branch {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target: Oid,
        merge_target: Option<Oid>,
        name: String,
        persistence: u8,
        is_remote: bool,
        is_merged: bool,
        is_tag: bool,
        svg_props: BranchSvgProps,
        end_index: Option<usize>,
    ) -> Self {
        Branch {
            target,
            merge_target,
            target_branch: None,
            source_branch: None,
            name,
            persistence,
            is_remote,
            is_merged,
            is_tag,
            svg_props,
            range: (end_index, None),
        }
    }
}

impl BranchSvgProps {
    pub fn new(order_group: usize, colour: String) -> Self {
        BranchSvgProps {
            order_group,
            target_order_group: None,
            source_order_group: None,
            colour,
            column: None,
            lines: Vec::new(),
            points: Vec::new(),
        }
    }
}
