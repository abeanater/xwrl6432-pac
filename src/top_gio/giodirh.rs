#[doc = "Register `GIODIRH` reader"]
pub type R = crate::R<GiodirhSpec>;
#[doc = "Register `GIODIRH` writer"]
pub type W = crate::W<GiodirhSpec>;
#[doc = "Field `GIODIRH` reader - 7:0\\]
GIO data direction of pins in Port H"]
pub type GiodirhR = crate::FieldReader;
#[doc = "Field `GIODIRH` writer - 7:0\\]
GIO data direction of pins in Port H"]
pub type GiodirhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU10` reader - 31:8\\]
Reserved"]
pub type Nu10R = crate::FieldReader<u32>;
#[doc = "Field `NU10` writer - 31:8\\]
Reserved"]
pub type Nu10W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port H"]
    #[inline(always)]
    pub fn giodirh(&self) -> GiodirhR {
        GiodirhR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu10(&self) -> Nu10R {
        Nu10R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port H"]
    #[inline(always)]
    #[must_use]
    pub fn giodirh(&mut self) -> GiodirhW<GiodirhSpec> {
        GiodirhW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu10(&mut self) -> Nu10W<GiodirhSpec> {
        Nu10W::new(self, 8)
    }
}
#[doc = "GIO data direction of pins in Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodirhSpec;
impl crate::RegisterSpec for GiodirhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodirh::R`](R) reader structure"]
impl crate::Readable for GiodirhSpec {}
#[doc = "`write(|w| ..)` method takes [`giodirh::W`](W) writer structure"]
impl crate::Writable for GiodirhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODIRH to value 0"]
impl crate::Resettable for GiodirhSpec {
    const RESET_VALUE: u32 = 0;
}
