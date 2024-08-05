#[doc = "Register `GIOSRCF` reader"]
pub type R = crate::R<GiosrcfSpec>;
#[doc = "Register `GIOSRCF` writer"]
pub type W = crate::W<GiosrcfSpec>;
#[doc = "Field `GIOSRCF` reader - 7:0\\]
GIO slew rate control for port F"]
pub type GiosrcfR = crate::FieldReader;
#[doc = "Field `GIOSRCF` writer - 7:0\\]
GIO slew rate control for port F"]
pub type GiosrcfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU40` reader - 31:8\\]
Reserved"]
pub type Nu40R = crate::FieldReader<u32>;
#[doc = "Field `NU40` writer - 31:8\\]
Reserved"]
pub type Nu40W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO slew rate control for port F"]
    #[inline(always)]
    pub fn giosrcf(&self) -> GiosrcfR {
        GiosrcfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu40(&self) -> Nu40R {
        Nu40R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO slew rate control for port F"]
    #[inline(always)]
    #[must_use]
    pub fn giosrcf(&mut self) -> GiosrcfW<GiosrcfSpec> {
        GiosrcfW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu40(&mut self) -> Nu40W<GiosrcfSpec> {
        Nu40W::new(self, 8)
    }
}
#[doc = "GIO slew rate select for port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrcf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrcf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosrcfSpec;
impl crate::RegisterSpec for GiosrcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giosrcf::R`](R) reader structure"]
impl crate::Readable for GiosrcfSpec {}
#[doc = "`write(|w| ..)` method takes [`giosrcf::W`](W) writer structure"]
impl crate::Writable for GiosrcfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSRCF to value 0"]
impl crate::Resettable for GiosrcfSpec {
    const RESET_VALUE: u32 = 0;
}
