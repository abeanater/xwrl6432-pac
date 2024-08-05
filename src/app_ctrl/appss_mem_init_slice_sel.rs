#[doc = "Register `APPSS_MEM_INIT_SLICE_SEL` reader"]
pub type R = crate::R<AppssMemInitSliceSelSpec>;
#[doc = "Register `APPSS_MEM_INIT_SLICE_SEL` writer"]
pub type W = crate::W<AppssMemInitSliceSelSpec>;
#[doc = "Field `cfg_bank1` reader - 2:0\\]
Selects the APPSS RAM1 partition that needs to be initialized. More than 1 partitions can be selected for mem_init. Bit#0 : Selects RAM_1A (64KB) Bit#1 : Selects RAM_1B (64KB) Bit#2 : Selects RAM_1C (128KB) 1 => RAM selected for mem_init operation 0 => RAM not selected for mem_init operation."]
pub type CfgBank1R = crate::FieldReader;
#[doc = "Field `cfg_bank1` writer - 2:0\\]
Selects the APPSS RAM1 partition that needs to be initialized. More than 1 partitions can be selected for mem_init. Bit#0 : Selects RAM_1A (64KB) Bit#1 : Selects RAM_1B (64KB) Bit#2 : Selects RAM_1C (128KB) 1 => RAM selected for mem_init operation 0 => RAM not selected for mem_init operation."]
pub type CfgBank1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cfg_bank2` reader - 4:3\\]
Selects the APPSS RAM2 partition that needs to be initialized. More than 1 partitions can be selected for mem_init. Bit#0 : Selects RAM_2A (16KB) Bit#1 : Selects RAM_2B (112KB) 1 => RAM partition selected for mem_init operation 0 => RAM partition not selected for mem_init operation."]
pub type CfgBank2R = crate::FieldReader;
#[doc = "Field `cfg_bank2` writer - 4:3\\]
Selects the APPSS RAM2 partition that needs to be initialized. More than 1 partitions can be selected for mem_init. Bit#0 : Selects RAM_2A (16KB) Bit#1 : Selects RAM_2B (112KB) 1 => RAM partition selected for mem_init operation 0 => RAM partition not selected for mem_init operation."]
pub type CfgBank2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the APPSS RAM1 partition that needs to be initialized. More than 1 partitions can be selected for mem_init. Bit#0 : Selects RAM_1A (64KB) Bit#1 : Selects RAM_1B (64KB) Bit#2 : Selects RAM_1C (128KB) 1 => RAM selected for mem_init operation 0 => RAM not selected for mem_init operation."]
    #[inline(always)]
    pub fn cfg_bank1(&self) -> CfgBank1R {
        CfgBank1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Selects the APPSS RAM2 partition that needs to be initialized. More than 1 partitions can be selected for mem_init. Bit#0 : Selects RAM_2A (16KB) Bit#1 : Selects RAM_2B (112KB) 1 => RAM partition selected for mem_init operation 0 => RAM partition not selected for mem_init operation."]
    #[inline(always)]
    pub fn cfg_bank2(&self) -> CfgBank2R {
        CfgBank2R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the APPSS RAM1 partition that needs to be initialized. More than 1 partitions can be selected for mem_init. Bit#0 : Selects RAM_1A (64KB) Bit#1 : Selects RAM_1B (64KB) Bit#2 : Selects RAM_1C (128KB) 1 => RAM selected for mem_init operation 0 => RAM not selected for mem_init operation."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_bank1(&mut self) -> CfgBank1W<AppssMemInitSliceSelSpec> {
        CfgBank1W::new(self, 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Selects the APPSS RAM2 partition that needs to be initialized. More than 1 partitions can be selected for mem_init. Bit#0 : Selects RAM_2A (16KB) Bit#1 : Selects RAM_2B (112KB) 1 => RAM partition selected for mem_init operation 0 => RAM partition not selected for mem_init operation."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_bank2(&mut self) -> CfgBank2W<AppssMemInitSliceSelSpec> {
        CfgBank2W::new(self, 3)
    }
}
#[doc = "APPSS_MEM_INIT_SLICE_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mem_init_slice_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mem_init_slice_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssMemInitSliceSelSpec;
impl crate::RegisterSpec for AppssMemInitSliceSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_mem_init_slice_sel::R`](R) reader structure"]
impl crate::Readable for AppssMemInitSliceSelSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_mem_init_slice_sel::W`](W) writer structure"]
impl crate::Writable for AppssMemInitSliceSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_MEM_INIT_SLICE_SEL to value 0"]
impl crate::Resettable for AppssMemInitSliceSelSpec {
    const RESET_VALUE: u32 = 0;
}
