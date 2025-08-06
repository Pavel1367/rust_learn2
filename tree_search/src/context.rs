use std::collections::VecDeque;

// Определение узла бинарного дерева
#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn with_children(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        TreeNode { val, left, right }
    }
}

// ЗАДАЧА 1: Реализуйте поиск в глубину (DFS)
// Должна возвращать true, если элемент найден, false - если нет
pub fn dfs_search(root: &Option<Box<TreeNode>>, target: i32) -> bool {
    if let Some(node) = root {
        if node.val == target {
            return true;
        }
        if dfs_search(&node.left, target) {
            return true
        }
        if dfs_search(&node.right, target) {
            return true
        }
    }
    false
}
pub fn dfs_search_stack(root: &Option<Box<TreeNode>>, target: i32) -> bool {
    let mut stack = VecDeque::new();
    if let Some(node) = root {
        stack.push_back(node);
    }
    while stack.len() > 0 {
        if let Some(node) = stack.pop_back() {
           if node.val == target {
               return true;
           }
            if let Some(left) = &node.left {
                stack.push_back(&left)
            }
            if let Some(right) = &node.right {
                stack.push_back(&right)
            }
        }
    }
    false
}
// ЗАДАЧА 2: Реализуйте поиск в ширину (BFS)
// Должна возвращать true, если элемент найден, false - если нет
pub fn bfs_search(root: &Option<Box<TreeNode>>, target: i32) -> bool {
    let mut stack = VecDeque::new();
    if let Some(node) = root {
        stack.push_back(node);
    }
    while stack.len() > 0 {
        if let Some(node) = stack.pop_front() {
            if node.val == target {
                return true;
            }
            if let Some(left) = &node.left {
                stack.push_back(&left)
            }
            if let Some(right) = &node.right {
                stack.push_back(&right)
            }
        }
    }
    false
}

// Функция для создания тестового дерева:
//       1
//      / \
//     2   3
//    / \   \
//   4   5   6
//      /
//     7
pub fn create_test_tree() -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::with_children(
        1,
        Some(Box::new(TreeNode::with_children(
            2,
            Some(Box::new(TreeNode::new(4))),
            Some(Box::new(TreeNode::with_children(
                5,
                Some(Box::new(TreeNode::new(7))),
                None,
            ))),
        ))),
        Some(Box::new(TreeNode::with_children(
            3,
            None,
            Some(Box::new(TreeNode::new(6))),
        ))),
    )))
}

fn main() {
    let tree = create_test_tree();

    println!("Тестовое дерево создано!");
    println!("Структура дерева:");
    println!("       1");
    println!("      / \\");
    println!("     2   3");
    println!("    / \\   \\");
    println!("   4   5   6");
    println!("      /");
    println!("     7");
    println!();

    // Тестирование DFS
    println!("=== Тесты DFS ===");
    let test_cases = [1, 2, 3, 4, 5, 6, 7, 8, 0];

    for &target in &test_cases {
        let found = dfs_search(&tree, target);
        println!("DFS поиск {}: {}", target, if found { "найден" } else { "не найден" });
    }

    println!();

    // Тестирование BFS
    println!("=== Тесты BFS ===");
    for &target in &test_cases {
        let found = bfs_search(&tree, target);
        println!("BFS поиск {}: {}", target, if found { "найден" } else { "не найден" });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_existing_elements() {
        let tree = create_test_tree();
        assert!(dfs_search(&tree, 1));
        assert!(dfs_search(&tree, 2));
        assert!(dfs_search(&tree, 3));
        assert!(dfs_search(&tree, 4));
        assert!(dfs_search(&tree, 5));
        assert!(dfs_search(&tree, 6));
        assert!(dfs_search(&tree, 7));
    }

    #[test]
    fn test_dfs_non_existing_elements() {
        let tree = create_test_tree();
        assert!(!dfs_search(&tree, 0));
        assert!(!dfs_search(&tree, 8));
        assert!(!dfs_search(&tree, 10));
    }

    #[test]
    fn test_bfs_existing_elements() {
        let tree = create_test_tree();
        assert!(bfs_search(&tree, 1));
        assert!(bfs_search(&tree, 2));
        assert!(bfs_search(&tree, 3));
        assert!(bfs_search(&tree, 4));
        assert!(bfs_search(&tree, 5));
        assert!(bfs_search(&tree, 6));
        assert!(bfs_search(&tree, 7));
    }

    #[test]
    fn test_bfs_non_existing_elements() {
        let tree = create_test_tree();
        assert!(!bfs_search(&tree, 0));
        assert!(!bfs_search(&tree, 8));
        assert!(!bfs_search(&tree, 10));
    }

    #[test]
    fn test_empty_tree() {
        assert!(!dfs_search(&None, 1));
        assert!(!bfs_search(&None, 1));
    }
}