#[doc = "Register `WIC_STAT_CLR` reader"]
pub type R = crate::R<WicStatClrSpec>;
#[doc = "Register `WIC_STAT_CLR` writer"]
pub type W = crate::W<WicStatClrSpec>;
#[doc = "Field `wicstatclr` reader - 31:0\\]
1 => Writing 1 to this bit, will clear the WIC_STAT status register of the corresponding bit. Self-clearing 0 => Writing 0 to this bit, leavesWIC_STAT status register unchanged for the corresponding bit."]
pub type WicstatclrR = crate::FieldReader<u32>;
#[doc = "Field `wicstatclr` writer - 31:0\\]
1 => Writing 1 to this bit, will clear the WIC_STAT status register of the corresponding bit. Self-clearing 0 => Writing 0 to this bit, leavesWIC_STAT status register unchanged for the corresponding bit."]
pub type WicstatclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
1 => Writing 1 to this bit, will clear the WIC_STAT status register of the corresponding bit. Self-clearing 0 => Writing 0 to this bit, leavesWIC_STAT status register unchanged for the corresponding bit."]
    #[inline(always)]
    pub fn wicstatclr(&self) -> WicstatclrR {
        WicstatclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
1 => Writing 1 to this bit, will clear the WIC_STAT status register of the corresponding bit. Self-clearing 0 => Writing 0 to this bit, leavesWIC_STAT status register unchanged for the corresponding bit."]
    #[inline(always)]
    #[must_use]
    pub fn wicstatclr(&mut self) -> WicstatclrW<WicStatClrSpec> {
        WicstatclrW::new(self, 0)
    }
}
#[doc = "WIC_STAT_CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`wic_stat_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wic_stat_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WicStatClrSpec;
impl crate::RegisterSpec for WicStatClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wic_stat_clr::R`](R) reader structure"]
impl crate::Readable for WicStatClrSpec {}
#[doc = "`write(|w| ..)` method takes [`wic_stat_clr::W`](W) writer structure"]
impl crate::Writable for WicStatClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIC_STAT_CLR to value 0"]
impl crate::Resettable for WicStatClrSpec {
    const RESET_VALUE: u32 = 0;
}
