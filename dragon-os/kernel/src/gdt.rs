//! Global Descriptor Table

use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::instructions::segmentation::{CS, Segment};
use x86_64::instructions::tables::load_tss;
use lazy_static::lazy_static;

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        (gdt, Selectors { code_selector, data_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::tables::load_gdt;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(x86_64::structures::gdt::SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0)); // Dummy call or fix if TSS needed
        // For simplicity in this demo, we skip TSS loading as we don't have a TSS set up in GDT
        // But we MUST reload CS.
    }
}
