mod helpers;

use std::fs;

fn main() {
    println!("> Starting Jauri CLI v{}...", env!("CARGO_PKG_VERSION"));

    check_deps();

    print!("? What is the name of your project? ");
    let project_name = input!();

    if project_name.is_empty() {
        println!("  > A project name is required, please try again.");
        std::process::exit(1);
    }

    print!("? What is the package? ");
    let package = input!();

    if package.is_empty() {
        println!("  > A package is required, please try again.");
        std::process::exit(1);
    }

    println!("> Creating project...");
    create_project(project_name, package);
}

fn create_project(name: String, package: String) {
    let package_dir = package.replace(".", "/");
    let source_dir = format!("src/main/java/{}", package_dir);
    fs::create_dir_all(&source_dir).expect("Failed to create source directory");

    let main_file = format!("{}/Main.java", &source_dir);
    let pom_xml = "pom.xml";
    let main_java_file = r#"package {{PACKAGE}};

    import ai.cirkl.jauri.JWebview;

    public class Main {
        public static void main(String[] args) {
            JWebview.launch("{{NAME}}", 800, 600);
        }
    }
    "#.replace("{{PACKAGE}}", package.as_str()).replace("{{NAME}}", name.as_str());
    let pom_xml_file = r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>{{PACKAGE}}</groupId>
    <artifactId>{{NAME}}</artifactId>
    <version>1.0-SNAPSHOT</version>

    <properties>
        <maven.compiler.source>23</maven.compiler.source>
        <maven.compiler.target>23</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

</project>
"#.replace("{{PACKAGE}}", package.as_str()).replace("{{NAME}}", name.as_str());

    fs::write(main_file, main_java_file).expect("Failed to create main file");
    fs::write(pom_xml, pom_xml_file).expect("Failed to create pom.xml");

    fs::create_dir("src/main/resources").expect("Failed to create resources directory");

    sh! {
        "cd src/main/resources && npx sv create ."
    }

    println!("> Downloading Jauri...");
    sh! {
        "echo TODO: Upload Jauri to be curled or to maven central repository"
    }

    println!("> Project created");
}

fn check_deps() {
    println!("> Checking dependencies...");

    let npm = sh! { "which npm > /dev/null" };
    let java = sh! { "which java > /dev/null" };

    if npm == 0 {
        println!("  > npm found");
    } else {
        println!("  > npm not found");
    }

    if java == 0 {
        println!("  > java found");
    } else {
        println!("  > java not found");
    }

    if java == 0 && npm == 0 {
        println!("> All dependencies found");
    } else {
        println!("> cSome dependencies not found, please install them and try again.");
        std::process::exit(1);
    }
}
