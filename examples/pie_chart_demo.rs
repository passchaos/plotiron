use plotiron::*;

fn main() {
    println!("PlotIron - Pie Chart Example");
    
    // Example 1: Simple pie chart
    let values = vec![30.0, 25.0, 20.0, 15.0, 10.0];
    let labels = vec![
        "Product A".to_string(),
        "Product B".to_string(), 
        "Product C".to_string(),
        "Product D".to_string(),
        "Product E".to_string()
    ];
    
    let mut fig1 = figure();
    fig1.add_subplot()
        .pie(&values, Some(&labels))
        .set_title("Market Share Distribution");
    
    std::fs::write("output/pie_chart_simple.svg", fig1.to_svg()).expect("Failed to write pie chart to file");
    println!("Simple pie chart saved to pie_chart_simple.svg");
    
    // Example 2: Budget allocation pie chart
    let budget_values = vec![40.0, 25.0, 15.0, 12.0, 8.0];
    let budget_labels = vec![
        "Development".to_string(),
        "Marketing".to_string(),
        "Operations".to_string(), 
        "Research".to_string(),
        "Other".to_string()
    ];
    
    let mut fig2 = figure();
    fig2.add_subplot()
        .pie(&budget_values, Some(&budget_labels))
        .set_title("Budget Allocation 2024");
    
    std::fs::write("output/pie_chart_budget.svg", fig2.to_svg()).expect("Failed to write budget pie chart to file");
    println!("Budget pie chart saved to pie_chart_budget.svg");
    
    println!("Pie chart demo completed!");
}