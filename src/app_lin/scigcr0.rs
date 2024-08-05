#[doc = "Register `SCIGCR0` reader"]
pub type R = crate::R<Scigcr0Spec>;
#[doc = "Register `SCIGCR0` writer"]
pub type W = crate::W<Scigcr0Spec>;
#[doc = "Field `RESET` reader - 0:0\\]
This bit resets the SCI/LIN module. This bit is effective in LIN or SCI-compatible mode.. This bit affects the reset state of the SCI/LIN module. Writable Only in privilege mode"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - 0:0\\]
This bit resets the SCI/LIN module. This bit is effective in LIN or SCI-compatible mode.. This bit affects the reset state of the SCI/LIN module. Writable Only in privilege mode"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 31:16\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:16\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit resets the SCI/LIN module. This bit is effective in LIN or SCI-compatible mode.. This bit affects the reset state of the SCI/LIN module. Writable Only in privilege mode"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit resets the SCI/LIN module. This bit is effective in LIN or SCI-compatible mode.. This bit affects the reset state of the SCI/LIN module. Writable Only in privilege mode"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<Scigcr0Spec> {
        ResetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scigcr0Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "The SCIGCR0 register defines the module reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scigcr0Spec;
impl crate::RegisterSpec for Scigcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scigcr0::R`](R) reader structure"]
impl crate::Readable for Scigcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`scigcr0::W`](W) writer structure"]
impl crate::Writable for Scigcr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIGCR0 to value 0"]
impl crate::Resettable for Scigcr0Spec {
    const RESET_VALUE: u32 = 0;
}
