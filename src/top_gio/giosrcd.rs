#[doc = "Register `GIOSRCD` reader"]
pub type R = crate::R<GiosrcdSpec>;
#[doc = "Register `GIOSRCD` writer"]
pub type W = crate::W<GiosrcdSpec>;
#[doc = "Field `GIOSRCD` reader - 7:0\\]
GIO slew rate control for port D"]
pub type GiosrcdR = crate::FieldReader;
#[doc = "Field `GIOSRCD` writer - 7:0\\]
GIO slew rate control for port D"]
pub type GiosrcdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU38` reader - 31:8\\]
Reserved"]
pub type Nu38R = crate::FieldReader<u32>;
#[doc = "Field `NU38` writer - 31:8\\]
Reserved"]
pub type Nu38W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO slew rate control for port D"]
    #[inline(always)]
    pub fn giosrcd(&self) -> GiosrcdR {
        GiosrcdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu38(&self) -> Nu38R {
        Nu38R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO slew rate control for port D"]
    #[inline(always)]
    #[must_use]
    pub fn giosrcd(&mut self) -> GiosrcdW<GiosrcdSpec> {
        GiosrcdW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu38(&mut self) -> Nu38W<GiosrcdSpec> {
        Nu38W::new(self, 8)
    }
}
#[doc = "GIO slew rate select for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosrcdSpec;
impl crate::RegisterSpec for GiosrcdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giosrcd::R`](R) reader structure"]
impl crate::Readable for GiosrcdSpec {}
#[doc = "`write(|w| ..)` method takes [`giosrcd::W`](W) writer structure"]
impl crate::Writable for GiosrcdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSRCD to value 0"]
impl crate::Resettable for GiosrcdSpec {
    const RESET_VALUE: u32 = 0;
}
