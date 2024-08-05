#[doc = "Register `APPSS_TPTC_ECCAGGR_CLK_CNTRL` reader"]
pub type R = crate::R<AppssTptcEccaggrClkCntrlSpec>;
#[doc = "Register `APPSS_TPTC_ECCAGGR_CLK_CNTRL` writer"]
pub type W = crate::W<AppssTptcEccaggrClkCntrlSpec>;
#[doc = "Field `tptc_A0` reader - 0:0\\]
Writing '0' will gate the clock to TPTC_A0-FIFO during ECC-AGGR interaction(fault injection)"]
pub type TptcA0R = crate::BitReader;
#[doc = "Field `tptc_A0` writer - 0:0\\]
Writing '0' will gate the clock to TPTC_A0-FIFO during ECC-AGGR interaction(fault injection)"]
pub type TptcA0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_A1` reader - 1:1\\]
Writing '0' will gate the clock to TPTC_A1-FIFO during ECC-AGGR interaction(fault injection)"]
pub type TptcA1R = crate::BitReader;
#[doc = "Field `tptc_A1` writer - 1:1\\]
Writing '0' will gate the clock to TPTC_A1-FIFO during ECC-AGGR interaction(fault injection)"]
pub type TptcA1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_B0` reader - 2:2\\]
Writing '0' will gate the clock to TPTC_B0-FIFO during ECC-AGGR interaction(fault injection)"]
pub type TptcB0R = crate::BitReader;
#[doc = "Field `tptc_B0` writer - 2:2\\]
Writing '0' will gate the clock to TPTC_B0-FIFO during ECC-AGGR interaction(fault injection)"]
pub type TptcB0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing '0' will gate the clock to TPTC_A0-FIFO during ECC-AGGR interaction(fault injection)"]
    #[inline(always)]
    pub fn tptc_a0(&self) -> TptcA0R {
        TptcA0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing '0' will gate the clock to TPTC_A1-FIFO during ECC-AGGR interaction(fault injection)"]
    #[inline(always)]
    pub fn tptc_a1(&self) -> TptcA1R {
        TptcA1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing '0' will gate the clock to TPTC_B0-FIFO during ECC-AGGR interaction(fault injection)"]
    #[inline(always)]
    pub fn tptc_b0(&self) -> TptcB0R {
        TptcB0R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing '0' will gate the clock to TPTC_A0-FIFO during ECC-AGGR interaction(fault injection)"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0(&mut self) -> TptcA0W<AppssTptcEccaggrClkCntrlSpec> {
        TptcA0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing '0' will gate the clock to TPTC_A1-FIFO during ECC-AGGR interaction(fault injection)"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1(&mut self) -> TptcA1W<AppssTptcEccaggrClkCntrlSpec> {
        TptcA1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing '0' will gate the clock to TPTC_B0-FIFO during ECC-AGGR interaction(fault injection)"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0(&mut self) -> TptcB0W<AppssTptcEccaggrClkCntrlSpec> {
        TptcB0W::new(self, 2)
    }
}
#[doc = "APPSS_TPTC_ECCAGGR_CLK_CNTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tptc_eccaggr_clk_cntrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tptc_eccaggr_clk_cntrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssTptcEccaggrClkCntrlSpec;
impl crate::RegisterSpec for AppssTptcEccaggrClkCntrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_tptc_eccaggr_clk_cntrl::R`](R) reader structure"]
impl crate::Readable for AppssTptcEccaggrClkCntrlSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_tptc_eccaggr_clk_cntrl::W`](W) writer structure"]
impl crate::Writable for AppssTptcEccaggrClkCntrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_TPTC_ECCAGGR_CLK_CNTRL to value 0"]
impl crate::Resettable for AppssTptcEccaggrClkCntrlSpec {
    const RESET_VALUE: u32 = 0;
}
