#[doc = "Register `INTF_STATS_RESET_SW` reader"]
pub type R = crate::R<IntfStatsResetSwSpec>;
#[doc = "Register `INTF_STATS_RESET_SW` writer"]
pub type W = crate::W<IntfStatsResetSwSpec>;
#[doc = "Field `INTF_STATS_RESET_SW` reader - 0:0\\]
SW reset for Interference statistics module. Its a self clearing bit."]
pub type IntfStatsResetSwR = crate::BitReader;
#[doc = "Field `INTF_STATS_RESET_SW` writer - 0:0\\]
SW reset for Interference statistics module. Its a self clearing bit."]
pub type IntfStatsResetSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SW reset for Interference statistics module. Its a self clearing bit."]
    #[inline(always)]
    pub fn intf_stats_reset_sw(&self) -> IntfStatsResetSwR {
        IntfStatsResetSwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SW reset for Interference statistics module. Its a self clearing bit."]
    #[inline(always)]
    #[must_use]
    pub fn intf_stats_reset_sw(&mut self) -> IntfStatsResetSwW<IntfStatsResetSwSpec> {
        IntfStatsResetSwW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfStatsResetSwSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "INTF_STATS_RESET_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_reset_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_reset_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfStatsResetSwSpec;
impl crate::RegisterSpec for IntfStatsResetSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_stats_reset_sw::R`](R) reader structure"]
impl crate::Readable for IntfStatsResetSwSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_stats_reset_sw::W`](W) writer structure"]
impl crate::Writable for IntfStatsResetSwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_STATS_RESET_SW to value 0"]
impl crate::Resettable for IntfStatsResetSwSpec {
    const RESET_VALUE: u32 = 0;
}
