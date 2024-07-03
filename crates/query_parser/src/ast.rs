use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
pub enum SExpressionNode<'a> {
    RootNode(RootNode<'a>),
    QueryActionNode(QueryActionNode<'a>),
    AttributeSelectNode(AttribuetSelectNode<'a>),
    ASTSelectNode(ASTSelectNode<'a>),
    ArraySelectNode(ArraySelect<'a>),
    ParentSelectNode(ParentSelectNode<'a>),
    // leaf node
    Identifier(Identifier<'a>),
    NumberLiteral(NumberLiteral<'a>),
    StringLiteral(StringLiteral<'a>),
    BoolLiteral(BoolLiteral<'a>),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier<'a> {
    pub name: Cow<'a, str>
}
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral<'a> {
    pub value: Cow<'a, str>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BoolLiteral<'a> {
    pub raw_value: Cow<'a, str>,
    pub value: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral<'a> {
    pub raw_value: Cow<'a, str>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RootNode<'a> {
   pub children: Vec<SExpressionNode<'a>>
}
#[derive(Debug, Clone, PartialEq)]
pub enum DescriptionNode<'a> {
    ParentSelect(ParentSelectNode<'a>),
    AttributeSelect(AttribuetSelectNode<'a>),
    ArraySelect(ArraySelect<'a>),
    ASTSelect(ASTSelectNode<'a>),
}
#[derive(Debug, Clone, PartialEq)]
pub struct ParentSelectNode<'a> {
    pub target_node: ASTSelectNode<'a>, 
}
#[derive(Debug, Clone, PartialEq)]
pub struct AttribuetSelectNode<'a> {
    pub key: Cow<'a, str>,
    // value should be a literal
}
#[derive(Debug, Clone, PartialEq)]
pub enum ArraySelectKind {
    OneOf,
    All,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ArraySelect<'a> {
    pub key: Cow<'a, str>,
    pub kind: ArraySelectKind,
    pub node_description: Vec<DescriptionNode<'a>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ASTSelectNode<'a> {
    pub node_description: Vec<DescriptionNode<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryActionNode<'a> {
   pub target_node: ASTSelectNode<'a>,
}