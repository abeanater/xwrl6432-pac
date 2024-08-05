#[doc = "Register `APPSS_TPCC_B_ERRAGG_STATUS_RAW` reader"]
pub type R = crate::R<AppssTpccBErraggStatusRawSpec>;
#[doc = "Register `APPSS_TPCC_B_ERRAGG_STATUS_RAW` writer"]
pub type W = crate::W<AppssTpccBErraggStatusRawSpec>;
#[doc = "Field `tpcc_b_errint` reader - 0:0\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBErrintR = crate::BitReader;
#[doc = "Field `tpcc_b_errint` writer - 0:0\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBErrintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_mpint` reader - 1:1\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBMpintR = crate::BitReader;
#[doc = "Field `tpcc_b_mpint` writer - 1:1\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBMpintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0_err` reader - 2:2\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB0ErrR = crate::BitReader;
#[doc = "Field `tptc_b0_err` writer - 2:2\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b1_err` reader - 3:3\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB1ErrR = crate::BitReader;
#[doc = "Field `tptc_b1_err` writer - 3:3\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB1ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_par_err` reader - 4:4\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBParErrR = crate::BitReader;
#[doc = "Field `tpcc_b_par_err` writer - 4:4\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBParErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b1_write_access_error` reader - 14:14\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB1WriteAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_b1_write_access_error` writer - 14:14\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB1WriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_write_access_error` reader - 16:16\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBWriteAccessErrorR = crate::BitReader;
#[doc = "Field `tpcc_b_write_access_error` writer - 16:16\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBWriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0_write_access_error` reader - 17:17\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB0WriteAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_b0_write_access_error` writer - 17:17\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB0WriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_read_access_error` reader - 24:24\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBReadAccessErrorR = crate::BitReader;
#[doc = "Field `tpcc_b_read_access_error` writer - 24:24\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TpccBReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0_read_access_error` reader - 25:25\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB0ReadAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_b0_read_access_error` writer - 25:25\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB0ReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b1_read_access_error` reader - 26:26\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB1ReadAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_b1_read_access_error` writer - 26:26\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
pub type TptcB1ReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tpcc_b_errint(&self) -> TpccBErrintR {
        TpccBErrintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tpcc_b_mpint(&self) -> TpccBMpintR {
        TpccBMpintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tptc_b0_err(&self) -> TptcB0ErrR {
        TptcB0ErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tptc_b1_err(&self) -> TptcB1ErrR {
        TptcB1ErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tpcc_b_par_err(&self) -> TpccBParErrR {
        TpccBParErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tptc_b1_write_access_error(&self) -> TptcB1WriteAccessErrorR {
        TptcB1WriteAccessErrorR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tpcc_b_write_access_error(&self) -> TpccBWriteAccessErrorR {
        TpccBWriteAccessErrorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tptc_b0_write_access_error(&self) -> TptcB0WriteAccessErrorR {
        TptcB0WriteAccessErrorR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tpcc_b_read_access_error(&self) -> TpccBReadAccessErrorR {
        TpccBReadAccessErrorR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tptc_b0_read_access_error(&self) -> TptcB0ReadAccessErrorR {
        TptcB0ReadAccessErrorR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    pub fn tptc_b1_read_access_error(&self) -> TptcB1ReadAccessErrorR {
        TptcB1ReadAccessErrorR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_errint(&mut self) -> TpccBErrintW<AppssTpccBErraggStatusRawSpec> {
        TpccBErrintW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_mpint(&mut self) -> TpccBMpintW<AppssTpccBErraggStatusRawSpec> {
        TpccBMpintW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_err(&mut self) -> TptcB0ErrW<AppssTpccBErraggStatusRawSpec> {
        TptcB0ErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_err(&mut self) -> TptcB1ErrW<AppssTpccBErraggStatusRawSpec> {
        TptcB1ErrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_par_err(&mut self) -> TpccBParErrW<AppssTpccBErraggStatusRawSpec> {
        TpccBParErrW::new(self, 4)
    }
    #[doc = "Bit 14 - 14:14\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_write_access_error(
        &mut self,
    ) -> TptcB1WriteAccessErrorW<AppssTpccBErraggStatusRawSpec> {
        TptcB1WriteAccessErrorW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_write_access_error(
        &mut self,
    ) -> TpccBWriteAccessErrorW<AppssTpccBErraggStatusRawSpec> {
        TpccBWriteAccessErrorW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_write_access_error(
        &mut self,
    ) -> TptcB0WriteAccessErrorW<AppssTpccBErraggStatusRawSpec> {
        TptcB0WriteAccessErrorW::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
Raw Status of Error from TPCC_B. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_read_access_error(
        &mut self,
    ) -> TpccBReadAccessErrorW<AppssTpccBErraggStatusRawSpec> {
        TpccBReadAccessErrorW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_read_access_error(
        &mut self,
    ) -> TptcB0ReadAccessErrorW<AppssTpccBErraggStatusRawSpec> {
        TptcB0ReadAccessErrorW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Raw Status of Error from TPTC_B0. Set irrespective if the Interupt is masked or unmasked in TPCC_B_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_read_access_error(
        &mut self,
    ) -> TptcB1ReadAccessErrorW<AppssTpccBErraggStatusRawSpec> {
        TptcB1ReadAccessErrorW::new(self, 26)
    }
}
#[doc = "APPSS_TPCC_B_ERRAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_b_erragg_status_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_b_erragg_status_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssTpccBErraggStatusRawSpec;
impl crate::RegisterSpec for AppssTpccBErraggStatusRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_tpcc_b_erragg_status_raw::R`](R) reader structure"]
impl crate::Readable for AppssTpccBErraggStatusRawSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_tpcc_b_erragg_status_raw::W`](W) writer structure"]
impl crate::Writable for AppssTpccBErraggStatusRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_TPCC_B_ERRAGG_STATUS_RAW to value 0"]
impl crate::Resettable for AppssTpccBErraggStatusRawSpec {
    const RESET_VALUE: u32 = 0;
}
