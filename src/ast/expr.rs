use paste::paste;

macro_rules! expression_nodes_impl {
    ( $(
        $(#[doc = $node_doc:expr])*
        $name:ident {},
    )* ) => {
        paste! {
            pub trait ExpressionNodeVisitor {
                type Output;

                $(
                    fn [<visit_ $name:snake>](&mut self) -> Self::Output;
                )*

                fn visit(&mut self) -> Self::Output {
                    // self.visit_query()
                    todo!()
                }
            }
        }
    };

}

expression_nodes_impl! {

    /// Expression: OrExpression
    Expression {},

    /// OrExpression: Vec<XorExpression>
    OrExpression {},

    /// XorExpression: Vec<AndExpression>
    XorExpression {},

    /// AndExpression: Vec<NotExpression>
    AndExpression {},

    /// NotExpression: Not? ComparsionExpression
    NotExpression {},

    /// ComparsionExpression: AddOrSubtractExpression + Vec<PartialComparisonExpression>
    ComparisonExpression {},

    /// PartialComparisonExpression: = <> < > <= >= AddOrSubtractExpression
    PartialComparisonExpression {},

    /// AddOrSubtractExpression: MultiplyDivideModuloExpression (+/- MultiplyDivideModuloExpression)*
    AddOrSubtractExpression {},

    /// MultiplyDivideModuloExpression: PowerOfExpression (*///% PowerOfExpression)*
    MultiplyDivideModuloExpression {},

    /// PowerOfExpression: UnaryAddOrSubtractExpression (^ UnaryAddOrSubtractExpression)*
    PowerOfExpression {},

    /// UnaryAddOrSubtractExpression: (+/-)* StringListNullOperatorExpression
    UnaryAddOrSubtractExpression {},

    /// StringListNullOperatorExpression: PropertyOrLabelsExpression, (StringOperatorExpression|ListOperatorExpression|NullOperatorExpression)*
    StringListNullOperatorExpression {},

    /// PropertyOrLabelsExpression: Atom, (PropertyLookup)*, (NodeLabels)+
    PropertyOrLabelsExpression {},

    /// StringOperatorExpression: (STARTS WITH | ENDS WITH | CONTAINS)? PropertyOrLabelsExpression
    StringOperatorExpression {},

    /// ListOperatorExpression:
    ListOperatorExpression {},

    /// NullOperatorExpression: IS NULL/ IS NOT NULL.
    NullOperatorExpression {},
}
