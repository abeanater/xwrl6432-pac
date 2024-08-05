#[doc = "Register `GIOPULDISG` reader"]
pub type R = crate::R<GiopuldisgSpec>;
#[doc = "Register `GIOPULDISG` writer"]
pub type W = crate::W<GiopuldisgSpec>;
#[doc = "Field `GIOPULDISG` reader - 7:0\\]
GIO pull disable for port G"]
pub type GiopuldisgR = crate::FieldReader;
#[doc = "Field `GIOPULDISG` writer - 7:0\\]
GIO pull disable for port G"]
pub type GiopuldisgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU39` reader - 31:8\\]
Reserved"]
pub type Nu39R = crate::FieldReader<u32>;
#[doc = "Field `NU39` writer - 31:8\\]
Reserved"]
pub type Nu39W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port G"]
    #[inline(always)]
    pub fn giopuldisg(&self) -> GiopuldisgR {
        GiopuldisgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu39(&self) -> Nu39R {
        Nu39R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port G"]
    #[inline(always)]
    #[must_use]
    pub fn giopuldisg(&mut self) -> GiopuldisgW<GiopuldisgSpec> {
        GiopuldisgW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu39(&mut self) -> Nu39W<GiopuldisgSpec> {
        Nu39W::new(self, 8)
    }
}
#[doc = "GIO pul disable for port G\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopuldisgSpec;
impl crate::RegisterSpec for GiopuldisgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopuldisg::R`](R) reader structure"]
impl crate::Readable for GiopuldisgSpec {}
#[doc = "`write(|w| ..)` method takes [`giopuldisg::W`](W) writer structure"]
impl crate::Writable for GiopuldisgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPULDISG to value 0"]
impl crate::Resettable for GiopuldisgSpec {
    const RESET_VALUE: u32 = 0;
}
