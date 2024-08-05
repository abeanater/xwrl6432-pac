#[doc = "Register `GIOSRCA` reader"]
pub type R = crate::R<GiosrcaSpec>;
#[doc = "Register `GIOSRCA` writer"]
pub type W = crate::W<GiosrcaSpec>;
#[doc = "Field `GIOSRCA` reader - 7:0\\]
GIO slew rate control for port A"]
pub type GiosrcaR = crate::FieldReader;
#[doc = "Field `GIOSRCA` writer - 7:0\\]
GIO slew rate control for port A"]
pub type GiosrcaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU35` reader - 31:8\\]
Reserved"]
pub type Nu35R = crate::FieldReader<u32>;
#[doc = "Field `NU35` writer - 31:8\\]
Reserved"]
pub type Nu35W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO slew rate control for port A"]
    #[inline(always)]
    pub fn giosrca(&self) -> GiosrcaR {
        GiosrcaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu35(&self) -> Nu35R {
        Nu35R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO slew rate control for port A"]
    #[inline(always)]
    #[must_use]
    pub fn giosrca(&mut self) -> GiosrcaW<GiosrcaSpec> {
        GiosrcaW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu35(&mut self) -> Nu35W<GiosrcaSpec> {
        Nu35W::new(self, 8)
    }
}
#[doc = "GIO slew rate select for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giosrca::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giosrca::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiosrcaSpec;
impl crate::RegisterSpec for GiosrcaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giosrca::R`](R) reader structure"]
impl crate::Readable for GiosrcaSpec {}
#[doc = "`write(|w| ..)` method takes [`giosrca::W`](W) writer structure"]
impl crate::Writable for GiosrcaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOSRCA to value 0"]
impl crate::Resettable for GiosrcaSpec {
    const RESET_VALUE: u32 = 0;
}
