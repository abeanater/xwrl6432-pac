#[doc = "Register `MEMACCESSERR` reader"]
pub type R = crate::R<MemaccesserrSpec>;
#[doc = "Register `MEMACCESSERR` writer"]
pub type W = crate::W<MemaccesserrSpec>;
#[doc = "Field `ERRCODECLR` reader - 3:0\\]
Reserved.TI internal"]
pub type ErrcodeclrR = crate::FieldReader;
#[doc = "Field `ERRCODECLR` writer - 3:0\\]
Reserved.TI internal"]
pub type ErrcodeclrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ERRCODEMASK` reader - 11:8\\]
Reserved.TI internal"]
pub type ErrcodemaskR = crate::FieldReader;
#[doc = "Field `ERRCODEMASK` writer - 11:8\\]
Reserved.TI internal"]
pub type ErrcodemaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STATERRCODE` reader - 19:16\\]
Reserved.TI internal"]
pub type StaterrcodeR = crate::FieldReader;
#[doc = "Field `STATERRCODE` writer - 19:16\\]
Reserved.TI internal"]
pub type StaterrcodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU3` reader - "]
pub type Nu3R = crate::FieldReader<u16>;
#[doc = "Field `NU3` writer - "]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn errcodeclr(&self) -> ErrcodeclrR {
        ErrcodeclrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn errcodemask(&self) -> ErrcodemaskR {
        ErrcodemaskR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn staterrcode(&self) -> StaterrcodeR {
        StaterrcodeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn errcodeclr(&mut self) -> ErrcodeclrW<MemaccesserrSpec> {
        ErrcodeclrW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<MemaccesserrSpec> {
        Nu1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn errcodemask(&mut self) -> ErrcodemaskW<MemaccesserrSpec> {
        ErrcodemaskW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<MemaccesserrSpec> {
        Nu2W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn staterrcode(&mut self) -> StaterrcodeW<MemaccesserrSpec> {
        StaterrcodeW::new(self, 16)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<MemaccesserrSpec> {
        Nu3W::new(self, 20)
    }
}
#[doc = "MEMACCESSERR\n\nYou can [`read`](crate::Reg::read) this register and get [`memaccesserr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memaccesserr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemaccesserrSpec;
impl crate::RegisterSpec for MemaccesserrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memaccesserr::R`](R) reader structure"]
impl crate::Readable for MemaccesserrSpec {}
#[doc = "`write(|w| ..)` method takes [`memaccesserr::W`](W) writer structure"]
impl crate::Writable for MemaccesserrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMACCESSERR to value 0"]
impl crate::Resettable for MemaccesserrSpec {
    const RESET_VALUE: u32 = 0;
}
