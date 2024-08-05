#[doc = "Register `APPSS_TPCC_MEMINIT_DONE` reader"]
pub type R = crate::R<AppssTpccMeminitDoneSpec>;
#[doc = "Register `APPSS_TPCC_MEMINIT_DONE` writer"]
pub type W = crate::W<AppssTpccMeminitDoneSpec>;
#[doc = "Field `tpcc_a_meminit_done` reader - 0:0\\]
This field will be high once intialization of TPCCA is finished. Writing '1' would clear the bit"]
pub type TpccAMeminitDoneR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_done` writer - 0:0\\]
This field will be high once intialization of TPCCA is finished. Writing '1' would clear the bit"]
pub type TpccAMeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_meminit_done` reader - 16:16\\]
This field will be high once intialization of TPCCB is finished. Writing '1' would clear the bit"]
pub type TpccBMeminitDoneR = crate::BitReader;
#[doc = "Field `tpcc_b_meminit_done` writer - 16:16\\]
This field will be high once intialization of TPCCB is finished. Writing '1' would clear the bit"]
pub type TpccBMeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once intialization of TPCCA is finished. Writing '1' would clear the bit"]
    #[inline(always)]
    pub fn tpcc_a_meminit_done(&self) -> TpccAMeminitDoneR {
        TpccAMeminitDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
This field will be high once intialization of TPCCB is finished. Writing '1' would clear the bit"]
    #[inline(always)]
    pub fn tpcc_b_meminit_done(&self) -> TpccBMeminitDoneR {
        TpccBMeminitDoneR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once intialization of TPCCA is finished. Writing '1' would clear the bit"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_done(&mut self) -> TpccAMeminitDoneW<AppssTpccMeminitDoneSpec> {
        TpccAMeminitDoneW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
This field will be high once intialization of TPCCB is finished. Writing '1' would clear the bit"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_meminit_done(&mut self) -> TpccBMeminitDoneW<AppssTpccMeminitDoneSpec> {
        TpccBMeminitDoneW::new(self, 16)
    }
}
#[doc = "APPSS_TPCC_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_meminit_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_meminit_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssTpccMeminitDoneSpec;
impl crate::RegisterSpec for AppssTpccMeminitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_tpcc_meminit_done::R`](R) reader structure"]
impl crate::Readable for AppssTpccMeminitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_tpcc_meminit_done::W`](W) writer structure"]
impl crate::Writable for AppssTpccMeminitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_TPCC_MEMINIT_DONE to value 0"]
impl crate::Resettable for AppssTpccMeminitDoneSpec {
    const RESET_VALUE: u32 = 0;
}
