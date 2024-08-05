#[doc = "Register `GIOPSLD` reader"]
pub type R = crate::R<GiopsldSpec>;
#[doc = "Register `GIOPSLD` writer"]
pub type W = crate::W<GiopsldSpec>;
#[doc = "Field `GIOPSLD` reader - 7:0\\]
GIO pull select for port D"]
pub type GiopsldR = crate::FieldReader;
#[doc = "Field `GIOPSLD` writer - 7:0\\]
GIO pull select for port D"]
pub type GiopsldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU38` reader - 31:8\\]
Reserved"]
pub type Nu38R = crate::FieldReader<u32>;
#[doc = "Field `NU38` writer - 31:8\\]
Reserved"]
pub type Nu38W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull select for port D"]
    #[inline(always)]
    pub fn giopsld(&self) -> GiopsldR {
        GiopsldR::new((self.bits & 0xff) as u8)
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
GIO pull select for port D"]
    #[inline(always)]
    #[must_use]
    pub fn giopsld(&mut self) -> GiopsldW<GiopsldSpec> {
        GiopsldW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu38(&mut self) -> Nu38W<GiopsldSpec> {
        Nu38W::new(self, 8)
    }
}
#[doc = "GIO pul select for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giopsld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopsld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopsldSpec;
impl crate::RegisterSpec for GiopsldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopsld::R`](R) reader structure"]
impl crate::Readable for GiopsldSpec {}
#[doc = "`write(|w| ..)` method takes [`giopsld::W`](W) writer structure"]
impl crate::Writable for GiopsldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPSLD to value 0"]
impl crate::Resettable for GiopsldSpec {
    const RESET_VALUE: u32 = 0;
}
