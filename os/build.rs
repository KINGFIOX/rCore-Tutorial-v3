use std::fs::{read_dir, File};
use std::io::{Result, Write};

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<_> = read_dir("../user/src/bin")
        .unwrap()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext // 去后缀
        })
        .collect();
    apps.sort();

    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        apps.len() // .quad <几个app>
    )?;

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?; // .quad app_0_start
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?; // .quad app_<last>_end

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:"#,
            idx, app, TARGET_PATH
        )?;
    }
    Ok(())
}

// 下面是生成的 link_app.S 文件的内容
//
//     .align 3
//     .section .data
//     .global _num_app
// _num_app:
//     .quad 4
//     .quad app_0_start
//     .quad app_1_start
//     .quad app_2_start
//     .quad app_3_start
//     .quad app_3_end
//
//     .section .data
//     .global app_0_start
//     .global app_0_end
// app_0_start:
//     .incbin "../user/target/riscv64gc-unknown-none-elf/release/00power_3.bin"
// app_0_end:
//
//     .section .data
//     .global app_1_start
//     .global app_1_end
// app_1_start:
//     .incbin "../user/target/riscv64gc-unknown-none-elf/release/01power_5.bin"
// app_1_end:
//
//     .section .data
//     .global app_2_start
//     .global app_2_end
// app_2_start:
//     .incbin "../user/target/riscv64gc-unknown-none-elf/release/02power_7.bin"
// app_2_end:
//
//     .section .data
//     .global app_3_start
//     .global app_3_end
// app_3_start:
//     .incbin "../user/target/riscv64gc-unknown-none-elf/release/03sleep.bin"
// app_3_end:
