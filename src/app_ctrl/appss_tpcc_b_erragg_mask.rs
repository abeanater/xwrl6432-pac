#[doc = "Register `APPSS_TPCC_B_ERRAGG_MASK` reader"]
pub type R = crate::R<AppssTpccBErraggMaskSpec>;
#[doc = "Register `APPSS_TPCC_B_ERRAGG_MASK` writer"]
pub type W = crate::W<AppssTpccBErraggMaskSpec>;
#[doc = "Field `tpcc_b_errint` reader - 0:0\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBErrintR = crate::BitReader;
#[doc = "Field `tpcc_b_errint` writer - 0:0\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBErrintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_mpint` reader - 1:1\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBMpintR = crate::BitReader;
#[doc = "Field `tpcc_b_mpint` writer - 1:1\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBMpintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0_err` reader - 2:2\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB0ErrR = crate::BitReader;
#[doc = "Field `tptc_b0_err` writer - 2:2\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b1_err` reader - 3:3\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB1ErrR = crate::BitReader;
#[doc = "Field `tptc_b1_err` writer - 3:3\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB1ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_par_err` reader - 4:4\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBParErrR = crate::BitReader;
#[doc = "Field `tpcc_b_par_err` writer - 4:4\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBParErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b1_write_access_error` reader - 14:14\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB1WriteAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_b1_write_access_error` writer - 14:14\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB1WriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_write_access_error` reader - 16:16\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBWriteAccessErrorR = crate::BitReader;
#[doc = "Field `tpcc_b_write_access_error` writer - 16:16\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBWriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0_write_access_error` reader - 17:17\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB0WriteAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_b0_write_access_error` writer - 17:17\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB0WriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_read_access_error` reader - 24:24\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBReadAccessErrorR = crate::BitReader;
#[doc = "Field `tpcc_b_read_access_error` writer - 24:24\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TpccBReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0_read_access_error` reader - 25:25\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB0ReadAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_b0_read_access_error` writer - 25:25\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB0ReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b1_read_access_error` reader - 26:26\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB1ReadAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_b1_read_access_error` writer - 26:26\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
pub type TptcB1ReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tpcc_b_errint(&self) -> TpccBErrintR {
        TpccBErrintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tpcc_b_mpint(&self) -> TpccBMpintR {
        TpccBMpintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tptc_b0_err(&self) -> TptcB0ErrR {
        TptcB0ErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tptc_b1_err(&self) -> TptcB1ErrR {
        TptcB1ErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tpcc_b_par_err(&self) -> TpccBParErrR {
        TpccBParErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tptc_b1_write_access_error(&self) -> TptcB1WriteAccessErrorR {
        TptcB1WriteAccessErrorR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tpcc_b_write_access_error(&self) -> TpccBWriteAccessErrorR {
        TpccBWriteAccessErrorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tptc_b0_write_access_error(&self) -> TptcB0WriteAccessErrorR {
        TptcB0WriteAccessErrorR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tpcc_b_read_access_error(&self) -> TpccBReadAccessErrorR {
        TpccBReadAccessErrorR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tptc_b0_read_access_error(&self) -> TptcB0ReadAccessErrorR {
        TptcB0ReadAccessErrorR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    pub fn tptc_b1_read_access_error(&self) -> TptcB1ReadAccessErrorR {
        TptcB1ReadAccessErrorR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_errint(&mut self) -> TpccBErrintW<AppssTpccBErraggMaskSpec> {
        TpccBErrintW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_mpint(&mut self) -> TpccBMpintW<AppssTpccBErraggMaskSpec> {
        TpccBMpintW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_err(&mut self) -> TptcB0ErrW<AppssTpccBErraggMaskSpec> {
        TptcB0ErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_err(&mut self) -> TptcB1ErrW<AppssTpccBErraggMaskSpec> {
        TptcB1ErrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_par_err(&mut self) -> TpccBParErrW<AppssTpccBErraggMaskSpec> {
        TpccBParErrW::new(self, 4)
    }
    #[doc = "Bit 14 - 14:14\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_write_access_error(
        &mut self,
    ) -> TptcB1WriteAccessErrorW<AppssTpccBErraggMaskSpec> {
        TptcB1WriteAccessErrorW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_write_access_error(
        &mut self,
    ) -> TpccBWriteAccessErrorW<AppssTpccBErraggMaskSpec> {
        TpccBWriteAccessErrorW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_write_access_error(
        &mut self,
    ) -> TptcB0WriteAccessErrorW<AppssTpccBErraggMaskSpec> {
        TptcB0WriteAccessErrorW::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
Mask Error from TPCC_B to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_read_access_error(&mut self) -> TpccBReadAccessErrorW<AppssTpccBErraggMaskSpec> {
        TpccBReadAccessErrorW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_read_access_error(
        &mut self,
    ) -> TptcB0ReadAccessErrorW<AppssTpccBErraggMaskSpec> {
        TptcB0ReadAccessErrorW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Mask Error from TPTC_B0 to aggregated Error TPCC_B_ERRAGG 1 : Error is Masked 0 : Error is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_read_access_error(
        &mut self,
    ) -> TptcB1ReadAccessErrorW<AppssTpccBErraggMaskSpec> {
        TptcB1ReadAccessErrorW::new(self, 26)
    }
}
#[doc = "APPSS_TPCC_B_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tpcc_b_erragg_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tpcc_b_erragg_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssTpccBErraggMaskSpec;
impl crate::RegisterSpec for AppssTpccBErraggMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_tpcc_b_erragg_mask::R`](R) reader structure"]
impl crate::Readable for AppssTpccBErraggMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_tpcc_b_erragg_mask::W`](W) writer structure"]
impl crate::Writable for AppssTpccBErraggMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_TPCC_B_ERRAGG_MASK to value 0"]
impl crate::Resettable for AppssTpccBErraggMaskSpec {
    const RESET_VALUE: u32 = 0;
}
