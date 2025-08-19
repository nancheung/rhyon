use std::fmt::Debug;

/// 通用查询规约trait，支持逻辑组合
pub trait Specification<T>: Debug + Clone {
    /// 检查实体是否满足规约
    fn is_satisfied_by(&self, entity: &T) -> bool;
    
    /// 与另一个规约进行AND组合
    fn and<S>(self, other: S) -> AndSpecification<Self, S>
    where
        Self: Sized,
        S: Specification<T>,
    {
        AndSpecification::new(self, other)
    }
    
    /// 与另一个规约进行OR组合
    fn or<S>(self, other: S) -> OrSpecification<Self, S>
    where
        Self: Sized,
        S: Specification<T>,
    {
        OrSpecification::new(self, other)
    }
    
    /// 对当前规约进行NOT操作
    fn not(self) -> NotSpecification<Self>
    where
        Self: Sized,
    {
        NotSpecification::new(self)
    }
}

/// AND逻辑组合规约
#[derive(Debug, Clone)]
pub struct AndSpecification<L, R> {
    left: L,
    right: R,
}

impl<L, R> AndSpecification<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<T, L, R> Specification<T> for AndSpecification<L, R>
where
    L: Specification<T>,
    R: Specification<T>,
{
    fn is_satisfied_by(&self, entity: &T) -> bool {
        self.left.is_satisfied_by(entity) && self.right.is_satisfied_by(entity)
    }
}

/// OR逻辑组合规约
#[derive(Debug, Clone)]
pub struct OrSpecification<L, R> {
    left: L,
    right: R,
}

impl<L, R> OrSpecification<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<T, L, R> Specification<T> for OrSpecification<L, R>
where
    L: Specification<T>,
    R: Specification<T>,
{
    fn is_satisfied_by(&self, entity: &T) -> bool {
        self.left.is_satisfied_by(entity) || self.right.is_satisfied_by(entity)
    }
}

/// NOT逻辑规约
#[derive(Debug, Clone)]
pub struct NotSpecification<S> {
    spec: S,
}

impl<S> NotSpecification<S> {
    pub fn new(spec: S) -> Self {
        Self { spec }
    }
}

impl<T, S> Specification<T> for NotSpecification<S>
where
    S: Specification<T>,
{
    fn is_satisfied_by(&self, entity: &T) -> bool {
        !self.spec.is_satisfied_by(entity)
    }
}

/// 空规约（总是返回true）
#[derive(Debug, Clone)]
pub struct EmptySpecification;

impl<T> Specification<T> for EmptySpecification {
    fn is_satisfied_by(&self, _entity: &T) -> bool {
        true
    }
}