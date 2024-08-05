#[doc = "Register `GIODIRB` reader"]
pub type R = crate::R<GiodirbSpec>;
#[doc = "Register `GIODIRB` writer"]
pub type W = crate::W<GiodirbSpec>;
#[doc = "Field `GIODIRB` reader - 7:0\\]
GIO data direction of pins in Port B"]
pub type GiodirbR = crate::FieldReader;
#[doc = "Field `GIODIRB` writer - 7:0\\]
GIO data direction of pins in Port B"]
pub type GiodirbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU6` reader - 31:8\\]
Reserved"]
pub type Nu6R = crate::FieldReader<u32>;
#[doc = "Field `NU6` writer - 31:8\\]
Reserved"]
pub type Nu6W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port B"]
    #[inline(always)]
    pub fn giodirb(&self) -> GiodirbR {
        GiodirbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu6(&self) -> Nu6R {
        Nu6R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port B"]
    #[inline(always)]
    #[must_use]
    pub fn giodirb(&mut self) -> GiodirbW<GiodirbSpec> {
        GiodirbW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu6(&mut self) -> Nu6W<GiodirbSpec> {
        Nu6W::new(self, 8)
    }
}
#[doc = "GIO data direction of pins in Port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodirbSpec;
impl crate::RegisterSpec for GiodirbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodirb::R`](R) reader structure"]
impl crate::Readable for GiodirbSpec {}
#[doc = "`write(|w| ..)` method takes [`giodirb::W`](W) writer structure"]
impl crate::Writable for GiodirbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODIRB to value 0"]
impl crate::Resettable for GiodirbSpec {
    const RESET_VALUE: u32 = 0;
}
