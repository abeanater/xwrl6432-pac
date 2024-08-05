#[doc = "Register `APPSS_TPCC_MEMINIT_STATUS` reader"]
pub type R = crate::R<AppssTpccMeminitStatusSpec>;
#[doc = "Register `APPSS_TPCC_MEMINIT_STATUS` writer"]
pub type W = crate::W<AppssTpccMeminitStatusSpec>;
#[doc = "Field `tpcc_a_meminit_status` reader - 0:0\\]
1'b0: No initialization is happening for TPCCA 1'b1: Initialization is in progress for TPCCB"]
pub type TpccAMeminitStatusR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_status` writer - 0:0\\]
1'b0: No initialization is happening for TPCCA 1'b1: Initialization is in progress for TPCCB"]
pub type TpccAMeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_meminit_status` reader - 16:16\\]
1'b0: No initialization is happening for TPCCA 1'b1: Initialization is in progress for TPCCB"]
pub type TpccBMeminitStatusR = crate::BitReader;
#[doc = "Field `tpcc_b_meminit_status` writer - 16:16\\]
1'b0: No initialization is happening for TPCCA 1'b1: Initialization is in progress for TPCCB"]
pub type TpccBMeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for TPCCA 1'b1: Initialization is in progress for TPCCB"]
    #[inline(always)]
    pub fn tpcc_a_meminit_status(&self) -> TpccAMeminitStatusR {
        TpccAMeminitStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
1'b0: No initialization is happening for TPCCA 1'b1: Initialization is in progress for TPCCB"]
    #[inline(always)]
    pub fn tpcc_b_meminit_status(&self) -> TpccBMeminitStatusR {
        TpccBMeminitStatusR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for TPCCA 1'b1: Initialization is in progress for TPCCB"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_status(&mut self) -> TpccAMeminitStatusW<AppssTpccMeminitStatusSpec> {
        TpccAMeminitStatusW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
1'b0: No initialization is happening for TPCCA 1'b1: Initialization is in progress for TPCCB"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_meminit_status(&mut self) -> TpccBMeminitStatusW<AppssTpccMeminitStatusSpec> {
        TpccBMeminitStatusW::new(self, 16)
    }
}
#[doc = "APPSS_TPCC_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_meminit_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_meminit_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssTpccMeminitStatusSpec;
impl crate::RegisterSpec for AppssTpccMeminitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_tpcc_meminit_status::R`](R) reader structure"]
impl crate::Readable for AppssTpccMeminitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_tpcc_meminit_status::W`](W) writer structure"]
impl crate::Writable for AppssTpccMeminitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_TPCC_MEMINIT_STATUS to value 0"]
impl crate::Resettable for AppssTpccMeminitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
