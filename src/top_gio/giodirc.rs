#[doc = "Register `GIODIRC` reader"]
pub type R = crate::R<GiodircSpec>;
#[doc = "Register `GIODIRC` writer"]
pub type W = crate::W<GiodircSpec>;
#[doc = "Field `GIODIRC` reader - 7:0\\]
GIO data direction of pins in Port C"]
pub type GiodircR = crate::FieldReader;
#[doc = "Field `GIODIRC` writer - 7:0\\]
GIO data direction of pins in Port C"]
pub type GiodircW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU7` reader - 31:8\\]
Reserved"]
pub type Nu7R = crate::FieldReader<u32>;
#[doc = "Field `NU7` writer - 31:8\\]
Reserved"]
pub type Nu7W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port C"]
    #[inline(always)]
    pub fn giodirc(&self) -> GiodircR {
        GiodircR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu7(&self) -> Nu7R {
        Nu7R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port C"]
    #[inline(always)]
    #[must_use]
    pub fn giodirc(&mut self) -> GiodircW<GiodircSpec> {
        GiodircW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu7(&mut self) -> Nu7W<GiodircSpec> {
        Nu7W::new(self, 8)
    }
}
#[doc = "GIO data direction of pins in Port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodircSpec;
impl crate::RegisterSpec for GiodircSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodirc::R`](R) reader structure"]
impl crate::Readable for GiodircSpec {}
#[doc = "`write(|w| ..)` method takes [`giodirc::W`](W) writer structure"]
impl crate::Writable for GiodircSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODIRC to value 0"]
impl crate::Resettable for GiodircSpec {
    const RESET_VALUE: u32 = 0;
}
