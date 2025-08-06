mod context;


fn main() {
    let tree = context::create_test_tree();

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
        let found = context::dfs_search_stack(&tree, target);
        println!("DFS поиск {}: {}", target, if found { "найден" } else { "не найден" });
    }

    println!();

    // Тестирование BFS
    println!("=== Тесты BFS ===");
    for &target in &test_cases {
        let found = context::bfs_search(&tree, target);
        println!("BFS поиск {}: {}", target, if found { "найден" } else { "не найден" });
    }
}
