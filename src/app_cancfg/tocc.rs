#[doc = "Register `TOCC` reader"]
pub type R = crate::R<ToccSpec>;
#[doc = "Register `TOCC` writer"]
pub type W = crate::W<ToccSpec>;
#[doc = "Field `ETOC` reader - 0:0\\]
Enable Timeout Counter"]
pub type EtocR = crate::BitReader;
#[doc = "Field `ETOC` writer - 0:0\\]
Enable Timeout Counter"]
pub type EtocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS` reader - 2:1\\]
Timeout Select"]
pub type TosR = crate::FieldReader;
#[doc = "Field `TOS` writer - 2:1\\]
Timeout Select"]
pub type TosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU23` reader - 15:3\\]
Reserved"]
pub type Nu23R = crate::FieldReader<u16>;
#[doc = "Field `NU23` writer - 15:3\\]
Reserved"]
pub type Nu23W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `TOP` reader - 31:16\\]
Timeout Period"]
pub type TopR = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - 31:16\\]
Timeout Period"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> EtocR {
        EtocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TosR {
        TosR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Reserved"]
    #[inline(always)]
    pub fn nu23(&self) -> Nu23R {
        Nu23R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> EtocW<ToccSpec> {
        EtocW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Timeout Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TosW<ToccSpec> {
        TosW::new(self, 1)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu23(&mut self) -> Nu23W<ToccSpec> {
        Nu23W::new(self, 3)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Timeout Period"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TopW<ToccSpec> {
        TopW::new(self, 16)
    }
}
#[doc = "TOCC\n\nYou can [`read`](crate::Reg::read) this register and get [`tocc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToccSpec;
impl crate::RegisterSpec for ToccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocc::R`](R) reader structure"]
impl crate::Readable for ToccSpec {}
#[doc = "`write(|w| ..)` method takes [`tocc::W`](W) writer structure"]
impl crate::Writable for ToccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCC to value 0"]
impl crate::Resettable for ToccSpec {
    const RESET_VALUE: u32 = 0;
}
