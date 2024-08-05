#[doc = "Register `SCIRD` reader"]
pub type R = crate::R<ScirdSpec>;
#[doc = "Register `SCIRD` writer"]
pub type W = crate::W<ScirdSpec>;
#[doc = "Field `RD` reader - 7:0\\]
Contains received data."]
pub type RdR = crate::FieldReader;
#[doc = "Field `RD` writer - 7:0\\]
Contains received data."]
pub type RdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Contains received data."]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Contains received data."]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RdW<ScirdSpec> {
        RdW::new(self, 0)
    }
}
#[doc = "Receiver Data Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`scird::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scird::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScirdSpec;
impl crate::RegisterSpec for ScirdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scird::R`](R) reader structure"]
impl crate::Readable for ScirdSpec {}
#[doc = "`write(|w| ..)` method takes [`scird::W`](W) writer structure"]
impl crate::Writable for ScirdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIRD to value 0"]
impl crate::Resettable for ScirdSpec {
    const RESET_VALUE: u32 = 0;
}
