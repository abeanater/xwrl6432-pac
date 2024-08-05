#[doc = "Register `TPCC_PARITY_CTRL` reader"]
pub type R = crate::R<TpccParityCtrlSpec>;
#[doc = "Register `TPCC_PARITY_CTRL` writer"]
pub type W = crate::W<TpccParityCtrlSpec>;
#[doc = "Field `tpcc_a_parity_en` reader - 0:0\\]
writing 1'b1 enables parity for TPCC_A"]
pub type TpccAParityEnR = crate::BitReader;
#[doc = "Field `tpcc_a_parity_en` writer - 0:0\\]
writing 1'b1 enables parity for TPCC_A"]
pub type TpccAParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_parity_testen` reader - 4:4\\]
parity test enable for tpcc a"]
pub type TpccAParityTestenR = crate::BitReader;
#[doc = "Field `tpcc_a_parity_testen` writer - 4:4\\]
parity test enable for tpcc a"]
pub type TpccAParityTestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_parity_en` reader - 8:8\\]
parity en for tpcc b"]
pub type TpccBParityEnR = crate::BitReader;
#[doc = "Field `tpcc_b_parity_en` writer - 8:8\\]
parity en for tpcc b"]
pub type TpccBParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_parity_testen` reader - 12:12\\]
parity test enable for tpcc b"]
pub type TpccBParityTestenR = crate::BitReader;
#[doc = "Field `tpcc_b_parity_testen` writer - 12:12\\]
parity test enable for tpcc b"]
pub type TpccBParityTestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_parity_err_clr` reader - 16:16\\]
Write pulse bit field: parity clear bit. Writing 1'b1 will clear the tpcc_a_parity_addr"]
pub type TpccAParityErrClrR = crate::BitReader;
#[doc = "Field `tpcc_a_parity_err_clr` writer - 16:16\\]
Write pulse bit field: parity clear bit. Writing 1'b1 will clear the tpcc_a_parity_addr"]
pub type TpccAParityErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_parity_err_clr` reader - 20:20\\]
Write pulse bit field: parity clear bit. Writing 1'b1 will clear the tpcc_b_parity_addr"]
pub type TpccBParityErrClrR = crate::BitReader;
#[doc = "Field `tpcc_b_parity_err_clr` writer - 20:20\\]
Write pulse bit field: parity clear bit. Writing 1'b1 will clear the tpcc_b_parity_addr"]
pub type TpccBParityErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 enables parity for TPCC_A"]
    #[inline(always)]
    pub fn tpcc_a_parity_en(&self) -> TpccAParityEnR {
        TpccAParityEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
parity test enable for tpcc a"]
    #[inline(always)]
    pub fn tpcc_a_parity_testen(&self) -> TpccAParityTestenR {
        TpccAParityTestenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
parity en for tpcc b"]
    #[inline(always)]
    pub fn tpcc_b_parity_en(&self) -> TpccBParityEnR {
        TpccBParityEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
parity test enable for tpcc b"]
    #[inline(always)]
    pub fn tpcc_b_parity_testen(&self) -> TpccBParityTestenR {
        TpccBParityTestenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Write pulse bit field: parity clear bit. Writing 1'b1 will clear the tpcc_a_parity_addr"]
    #[inline(always)]
    pub fn tpcc_a_parity_err_clr(&self) -> TpccAParityErrClrR {
        TpccAParityErrClrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Write pulse bit field: parity clear bit. Writing 1'b1 will clear the tpcc_b_parity_addr"]
    #[inline(always)]
    pub fn tpcc_b_parity_err_clr(&self) -> TpccBParityErrClrR {
        TpccBParityErrClrR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 enables parity for TPCC_A"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_parity_en(&mut self) -> TpccAParityEnW<TpccParityCtrlSpec> {
        TpccAParityEnW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
parity test enable for tpcc a"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_parity_testen(&mut self) -> TpccAParityTestenW<TpccParityCtrlSpec> {
        TpccAParityTestenW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
parity en for tpcc b"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_parity_en(&mut self) -> TpccBParityEnW<TpccParityCtrlSpec> {
        TpccBParityEnW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
parity test enable for tpcc b"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_parity_testen(&mut self) -> TpccBParityTestenW<TpccParityCtrlSpec> {
        TpccBParityTestenW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Write pulse bit field: parity clear bit. Writing 1'b1 will clear the tpcc_a_parity_addr"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_parity_err_clr(&mut self) -> TpccAParityErrClrW<TpccParityCtrlSpec> {
        TpccAParityErrClrW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Write pulse bit field: parity clear bit. Writing 1'b1 will clear the tpcc_b_parity_addr"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_parity_err_clr(&mut self) -> TpccBParityErrClrW<TpccParityCtrlSpec> {
        TpccBParityErrClrW::new(self, 20)
    }
}
#[doc = "TPCC_PARITY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`tpcc_parity_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpcc_parity_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpccParityCtrlSpec;
impl crate::RegisterSpec for TpccParityCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpcc_parity_ctrl::R`](R) reader structure"]
impl crate::Readable for TpccParityCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tpcc_parity_ctrl::W`](W) writer structure"]
impl crate::Writable for TpccParityCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPCC_PARITY_CTRL to value 0"]
impl crate::Resettable for TpccParityCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
