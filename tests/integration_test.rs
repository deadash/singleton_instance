use singleton_macro::Instance;
use singleton_instance::Initializable;

#[derive(Instance)]
struct TestStruct {
    value: i32,
}

impl Initializable for TestStruct {
    fn initialize() -> Self {
        TestStruct { value: 42 }
    }
}

#[test]
fn integration_test_for_singleton() {
    let instance1 = TestStruct::instance();
    let instance2 = TestStruct::instance();
    
    // 验证两个实例是否指向同一内存地址（即是否为同一实例）
    assert_eq!(instance1 as *const _, instance2 as *const _);
    
    // 验证单例字段值是否正确
    assert_eq!((*instance1).value, 42);
    assert_eq!((*instance2).value, 42);

    // 修改后判断
    (*instance1).value = 43;

    // 验证单例字段值是否正确
    assert_eq!((*instance1).value, 43);
    assert_eq!((*instance2).value, 43);
}
