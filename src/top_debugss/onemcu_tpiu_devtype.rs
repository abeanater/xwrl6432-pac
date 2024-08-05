#[doc = "Register `ONEMCU_TPIU_DEVTYPE` reader"]
pub type R = crate::R<OnemcuTpiuDevtypeSpec>;
#[doc = "Register `ONEMCU_TPIU_DEVTYPE` writer"]
pub type W = crate::W<OnemcuTpiuDevtypeSpec>;
#[doc = "Field `ONEMCU_TPIU_DEVTYPE` reader - 31:0\\]
Device type identifier"]
pub type OnemcuTpiuDevtypeR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_DEVTYPE` writer - 31:0\\]
Device type identifier"]
pub type OnemcuTpiuDevtypeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Device type identifier"]
    #[inline(always)]
    pub fn onemcu_tpiu_devtype(&self) -> OnemcuTpiuDevtypeR {
        OnemcuTpiuDevtypeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Device type identifier"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_devtype(&mut self) -> OnemcuTpiuDevtypeW<OnemcuTpiuDevtypeSpec> {
        OnemcuTpiuDevtypeW::new(self, 0)
    }
}
#[doc = "Device type identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_devtype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_devtype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuDevtypeSpec;
impl crate::RegisterSpec for OnemcuTpiuDevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_devtype::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuDevtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_devtype::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuDevtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_DEVTYPE to value 0"]
impl crate::Resettable for OnemcuTpiuDevtypeSpec {
    const RESET_VALUE: u32 = 0;
}
