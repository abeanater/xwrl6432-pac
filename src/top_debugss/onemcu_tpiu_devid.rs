#[doc = "Register `ONEMCU_TPIU_DEVID` reader"]
pub type R = crate::R<OnemcuTpiuDevidSpec>;
#[doc = "Register `ONEMCU_TPIU_DEVID` writer"]
pub type W = crate::W<OnemcuTpiuDevidSpec>;
#[doc = "Field `ONEMCU_TPIU_DEVID` reader - 31:0\\]
Device ID"]
pub type OnemcuTpiuDevidR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_DEVID` writer - 31:0\\]
Device ID"]
pub type OnemcuTpiuDevidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Device ID"]
    #[inline(always)]
    pub fn onemcu_tpiu_devid(&self) -> OnemcuTpiuDevidR {
        OnemcuTpiuDevidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Device ID"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_devid(&mut self) -> OnemcuTpiuDevidW<OnemcuTpiuDevidSpec> {
        OnemcuTpiuDevidW::new(self, 0)
    }
}
#[doc = "Device ID\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_devid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_devid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuDevidSpec;
impl crate::RegisterSpec for OnemcuTpiuDevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_devid::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuDevidSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_devid::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuDevidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_DEVID to value 0"]
impl crate::Resettable for OnemcuTpiuDevidSpec {
    const RESET_VALUE: u32 = 0;
}
