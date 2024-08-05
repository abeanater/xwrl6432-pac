#[doc = "Register `APPSS_MCANA_INT_STAT` reader"]
pub type R = crate::R<AppssMcanaIntStatSpec>;
#[doc = "Register `APPSS_MCANA_INT_STAT` writer"]
pub type W = crate::W<AppssMcanaIntStatSpec>;
#[doc = "Field `mcan_int_status` reader - 31:0\\]
Interrupt status for 32 MCANSS TX DMA interrupts. 1'b1 in bit&lt;0-31> gives pending status for interrupt &lt;0-31> respectively in MCANA"]
pub type McanIntStatusR = crate::FieldReader<u32>;
#[doc = "Field `mcan_int_status` writer - 31:0\\]
Interrupt status for 32 MCANSS TX DMA interrupts. 1'b1 in bit&lt;0-31> gives pending status for interrupt &lt;0-31> respectively in MCANA"]
pub type McanIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt status for 32 MCANSS TX DMA interrupts. 1'b1 in bit&lt;0-31> gives pending status for interrupt &lt;0-31> respectively in MCANA"]
    #[inline(always)]
    pub fn mcan_int_status(&self) -> McanIntStatusR {
        McanIntStatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt status for 32 MCANSS TX DMA interrupts. 1'b1 in bit&lt;0-31> gives pending status for interrupt &lt;0-31> respectively in MCANA"]
    #[inline(always)]
    #[must_use]
    pub fn mcan_int_status(&mut self) -> McanIntStatusW<AppssMcanaIntStatSpec> {
        McanIntStatusW::new(self, 0)
    }
}
#[doc = "APPSS_MCANA_INT_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mcana_int_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mcana_int_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssMcanaIntStatSpec;
impl crate::RegisterSpec for AppssMcanaIntStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_mcana_int_stat::R`](R) reader structure"]
impl crate::Readable for AppssMcanaIntStatSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_mcana_int_stat::W`](W) writer structure"]
impl crate::Writable for AppssMcanaIntStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_MCANA_INT_STAT to value 0"]
impl crate::Resettable for AppssMcanaIntStatSpec {
    const RESET_VALUE: u32 = 0;
}
