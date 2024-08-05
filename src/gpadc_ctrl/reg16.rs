#[doc = "Register `REG16` reader"]
pub type R = crate::R<Reg16Spec>;
#[doc = "Register `REG16` writer"]
pub type W = crate::W<Reg16Spec>;
#[doc = "Field `GPADC_MEM_INIT_DONE_STAT` reader - 0:0\\]
Status for Data Mem init done.Used for FW polling .Will read '0' when init process is under progress"]
pub type GpadcMemInitDoneStatR = crate::BitReader;
#[doc = "Field `GPADC_MEM_INIT_DONE_STAT` writer - 0:0\\]
Status for Data Mem init done.Used for FW polling .Will read '0' when init process is under progress"]
pub type GpadcMemInitDoneStatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:1\\]
TI reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:1\\]
TI reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status for Data Mem init done.Used for FW polling .Will read '0' when init process is under progress"]
    #[inline(always)]
    pub fn gpadc_mem_init_done_stat(&self) -> GpadcMemInitDoneStatR {
        GpadcMemInitDoneStatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
TI reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status for Data Mem init done.Used for FW polling .Will read '0' when init process is under progress"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_mem_init_done_stat(&mut self) -> GpadcMemInitDoneStatW<Reg16Spec> {
        GpadcMemInitDoneStatW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Reg16Spec> {
        NuW::new(self, 1)
    }
}
#[doc = "REG16\n\nYou can [`read`](crate::Reg::read) this register and get [`reg16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg16Spec;
impl crate::RegisterSpec for Reg16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg16::R`](R) reader structure"]
impl crate::Readable for Reg16Spec {}
#[doc = "`write(|w| ..)` method takes [`reg16::W`](W) writer structure"]
impl crate::Writable for Reg16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG16 to value 0"]
impl crate::Resettable for Reg16Spec {
    const RESET_VALUE: u32 = 0;
}
