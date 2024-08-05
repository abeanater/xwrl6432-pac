#[doc = "Register `FECSS_CLK_GATE` reader"]
pub type R = crate::R<FecssClkGateSpec>;
#[doc = "Register `FECSS_CLK_GATE` writer"]
pub type W = crate::W<FecssClkGateSpec>;
#[doc = "Field `grp1` reader - 2:0\\]
Multibit: Writing 3'b111 will gate FEC_SYS_CLK and FECSS peripheral clocks except DFE and Timing Engine"]
pub type Grp1R = crate::FieldReader;
#[doc = "Field `grp1` writer - 2:0\\]
Multibit: Writing 3'b111 will gate FEC_SYS_CLK and FECSS peripheral clocks except DFE and Timing Engine"]
pub type Grp1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp2` reader - 5:3\\]
Multibit: Writing 3'b111 will gate ADC_CLK going to DFE and Timing Engine"]
pub type Grp2R = crate::FieldReader;
#[doc = "Field `grp2` writer - 5:3\\]
Multibit: Writing 3'b111 will gate ADC_CLK going to DFE and Timing Engine"]
pub type Grp2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Multibit: Writing 3'b111 will gate FEC_SYS_CLK and FECSS peripheral clocks except DFE and Timing Engine"]
    #[inline(always)]
    pub fn grp1(&self) -> Grp1R {
        Grp1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Multibit: Writing 3'b111 will gate ADC_CLK going to DFE and Timing Engine"]
    #[inline(always)]
    pub fn grp2(&self) -> Grp2R {
        Grp2R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Multibit: Writing 3'b111 will gate FEC_SYS_CLK and FECSS peripheral clocks except DFE and Timing Engine"]
    #[inline(always)]
    #[must_use]
    pub fn grp1(&mut self) -> Grp1W<FecssClkGateSpec> {
        Grp1W::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Multibit: Writing 3'b111 will gate ADC_CLK going to DFE and Timing Engine"]
    #[inline(always)]
    #[must_use]
    pub fn grp2(&mut self) -> Grp2W<FecssClkGateSpec> {
        Grp2W::new(self, 3)
    }
}
#[doc = "FECSS_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`fecss_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fecss_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecssClkGateSpec;
impl crate::RegisterSpec for FecssClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fecss_clk_gate::R`](R) reader structure"]
impl crate::Readable for FecssClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`fecss_clk_gate::W`](W) writer structure"]
impl crate::Writable for FecssClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FECSS_CLK_GATE to value 0"]
impl crate::Resettable for FecssClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
