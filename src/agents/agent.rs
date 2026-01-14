#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentName {
    DGLAWSSIT,
    DGLAWSUAT,
    DGLGCPSIT,
    DGLGCPUAT,
}

impl AgentName {
    pub fn from_string(s: String) -> Self {
        match s.as_str() {
            "dgl-aws-sit" => AgentName::DGLAWSSIT,
            "dgl-aws-uat" => AgentName::DGLAWSUAT,
            "dgl-gcp-sit" => AgentName::DGLGCPSIT,
            "dgl-gcp-uat" => AgentName::DGLGCPUAT,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for AgentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentName::DGLAWSSIT => write!(f, "dgl-aws-sit"),
            AgentName::DGLAWSUAT => write!(f, "dgl-aws-uat"),
            AgentName::DGLGCPSIT => write!(f, "dgl-gcp-sit"),
            AgentName::DGLGCPUAT => write!(f, "dgl-gcp-uat"),
        }
    }
}
