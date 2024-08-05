#[doc = "Register `GIOEMUA` reader"]
pub type R = crate::R<GioemuaSpec>;
#[doc = "Register `GIOEMUA` writer"]
pub type W = crate::W<GioemuaSpec>;
#[doc = "Field `GIOEMUA` reader - 5:0\\]
GIO emulation register A"]
pub type GioemuaR = crate::FieldReader;
#[doc = "Field `GIOEMUA` writer - 5:0\\]
GIO emulation register A"]
pub type GioemuaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU3` reader - 31:6\\]
Reserved"]
pub type Nu3R = crate::FieldReader<u32>;
#[doc = "Field `NU3` writer - 31:6\\]
Reserved"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
GIO emulation register A"]
    #[inline(always)]
    pub fn gioemua(&self) -> GioemuaR {
        GioemuaR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
GIO emulation register A"]
    #[inline(always)]
    #[must_use]
    pub fn gioemua(&mut self) -> GioemuaW<GioemuaSpec> {
        GioemuaW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<GioemuaSpec> {
        Nu3W::new(self, 6)
    }
}
#[doc = "GIO emulation register A\n\nYou can [`read`](crate::Reg::read) this register and get [`gioemua::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioemua::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioemuaSpec;
impl crate::RegisterSpec for GioemuaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioemua::R`](R) reader structure"]
impl crate::Readable for GioemuaSpec {}
#[doc = "`write(|w| ..)` method takes [`gioemua::W`](W) writer structure"]
impl crate::Writable for GioemuaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOEMUA to value 0"]
impl crate::Resettable for GioemuaSpec {
    const RESET_VALUE: u32 = 0;
}
