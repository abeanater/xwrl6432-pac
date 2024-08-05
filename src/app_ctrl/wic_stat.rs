#[doc = "Register `WIC_STAT` reader"]
pub type R = crate::R<WicStatSpec>;
#[doc = "Register `WIC_STAT` writer"]
pub type W = crate::W<WicStatSpec>;
#[doc = "Field `wicstat` reader - 31:0\\]
1 => Interrupt bit set. The interupt bit is sticky bit. Should be cleared using WIC_STAT_CLR register or subsystem reset. 0 -> Interrupt bit not set. Sticky bits keep their value when they changed to logical 1 and is cleared only by writing 1 to WIC_STAT_CLR register."]
pub type WicstatR = crate::FieldReader<u32>;
#[doc = "Field `wicstat` writer - 31:0\\]
1 => Interrupt bit set. The interupt bit is sticky bit. Should be cleared using WIC_STAT_CLR register or subsystem reset. 0 -> Interrupt bit not set. Sticky bits keep their value when they changed to logical 1 and is cleared only by writing 1 to WIC_STAT_CLR register."]
pub type WicstatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
1 => Interrupt bit set. The interupt bit is sticky bit. Should be cleared using WIC_STAT_CLR register or subsystem reset. 0 -> Interrupt bit not set. Sticky bits keep their value when they changed to logical 1 and is cleared only by writing 1 to WIC_STAT_CLR register."]
    #[inline(always)]
    pub fn wicstat(&self) -> WicstatR {
        WicstatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
1 => Interrupt bit set. The interupt bit is sticky bit. Should be cleared using WIC_STAT_CLR register or subsystem reset. 0 -> Interrupt bit not set. Sticky bits keep their value when they changed to logical 1 and is cleared only by writing 1 to WIC_STAT_CLR register."]
    #[inline(always)]
    #[must_use]
    pub fn wicstat(&mut self) -> WicstatW<WicStatSpec> {
        WicstatW::new(self, 0)
    }
}
#[doc = "WIC_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`wic_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wic_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WicStatSpec;
impl crate::RegisterSpec for WicStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wic_stat::R`](R) reader structure"]
impl crate::Readable for WicStatSpec {}
#[doc = "`write(|w| ..)` method takes [`wic_stat::W`](W) writer structure"]
impl crate::Writable for WicStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIC_STAT to value 0"]
impl crate::Resettable for WicStatSpec {
    const RESET_VALUE: u32 = 0;
}
