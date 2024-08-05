#[doc = "Register `APPSS_TPCC_MEMINIT_START` reader"]
pub type R = crate::R<AppssTpccMeminitStartSpec>;
#[doc = "Register `APPSS_TPCC_MEMINIT_START` writer"]
pub type W = crate::W<AppssTpccMeminitStartSpec>;
#[doc = "Field `tpcc_a_meminit_start` reader - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the TPCCA"]
pub type TpccAMeminitStartR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_start` writer - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the TPCCA"]
pub type TpccAMeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_meminit_start` reader - 16:16\\]
Write_pulse bit field: Writing 1'b1 will start initializing the TPCCB"]
pub type TpccBMeminitStartR = crate::BitReader;
#[doc = "Field `tpcc_b_meminit_start` writer - 16:16\\]
Write_pulse bit field: Writing 1'b1 will start initializing the TPCCB"]
pub type TpccBMeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the TPCCA"]
    #[inline(always)]
    pub fn tpcc_a_meminit_start(&self) -> TpccAMeminitStartR {
        TpccAMeminitStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Write_pulse bit field: Writing 1'b1 will start initializing the TPCCB"]
    #[inline(always)]
    pub fn tpcc_b_meminit_start(&self) -> TpccBMeminitStartR {
        TpccBMeminitStartR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the TPCCA"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_start(&mut self) -> TpccAMeminitStartW<AppssTpccMeminitStartSpec> {
        TpccAMeminitStartW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Write_pulse bit field: Writing 1'b1 will start initializing the TPCCB"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_meminit_start(&mut self) -> TpccBMeminitStartW<AppssTpccMeminitStartSpec> {
        TpccBMeminitStartW::new(self, 16)
    }
}
#[doc = "APPSS_TPCC_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_meminit_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_meminit_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssTpccMeminitStartSpec;
impl crate::RegisterSpec for AppssTpccMeminitStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_tpcc_meminit_start::R`](R) reader structure"]
impl crate::Readable for AppssTpccMeminitStartSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_tpcc_meminit_start::W`](W) writer structure"]
impl crate::Writable for AppssTpccMeminitStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_TPCC_MEMINIT_START to value 0"]
impl crate::Resettable for AppssTpccMeminitStartSpec {
    const RESET_VALUE: u32 = 0;
}
