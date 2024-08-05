#[doc = "Register `GIOSRCB` reader"]
pub type R = crate::R<GiosrcbSpec>;
#[doc = "Register `GIOSRCB` writer"]
pub type W = crate::W<GiosrcbSpec>;
#[doc = "Field `GIOSRCB` reader - 7:0\\]
GIO slew rate control for port B"]
pub type GiosrcbR = crate::FieldReader;
#[doc = "Field `GIOSRCB` writer - 7:0\\]
GIO slew rate control for port B"]
pub type GiosrcbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU36` reader - 31:8\\]
Reserved"]
pub type Nu36R = crate::FieldReader<u32>;
#[doc = "Field `NU36` writer - 31:8\\]
Reserved"]
pub type Nu36W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO slew rate control for port B"]
    #[inline(always)]
    pub fn giosrcb(&self) -> GiosrcbR {
        GiosrcbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu36(&self) -> Nu36R {
        Nu36R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO slew rate control for port B"]
    #[inline(always)]
    #[must_use]
    pub fn giosrcb(&mut self) -> GiosrcbW<GiosrcbSpec> {
        GiosrcbW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu36(&mut self) -> Nu36W<GiosrcbSpec> {
        Nu36W::new(self, 8)
    }
}
#[doc = "GIO slew rate select for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrcb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrcb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosrcbSpec;
impl crate::RegisterSpec for GiosrcbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giosrcb::R`](R) reader structure"]
impl crate::Readable for GiosrcbSpec {}
#[doc = "`write(|w| ..)` method takes [`giosrcb::W`](W) writer structure"]
impl crate::Writable for GiosrcbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSRCB to value 0"]
impl crate::Resettable for GiosrcbSpec {
    const RESET_VALUE: u32 = 0;
}
