#[doc = "Register `TXEFS` reader"]
pub type R = crate::R<TxefsSpec>;
#[doc = "Register `TXEFS` writer"]
pub type W = crate::W<TxefsSpec>;
#[doc = "Field `EFFL` reader - 5:0\\]
Event FIFO FIll Level"]
pub type EfflR = crate::FieldReader;
#[doc = "Field `EFFL` writer - 5:0\\]
Event FIFO FIll Level"]
pub type EfflW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU69` reader - 7:6\\]
Reserved"]
pub type Nu69R = crate::FieldReader;
#[doc = "Field `NU69` writer - 7:6\\]
Reserved"]
pub type Nu69W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EFGI` reader - 12:8\\]
Event FIFO Get Index"]
pub type EfgiR = crate::FieldReader;
#[doc = "Field `EFGI` writer - 12:8\\]
Event FIFO Get Index"]
pub type EfgiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU70` reader - 15:13\\]
Reserved"]
pub type Nu70R = crate::FieldReader;
#[doc = "Field `NU70` writer - 15:13\\]
Reserved"]
pub type Nu70W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EFPI` reader - 20:16\\]
Event FIFO Put Index"]
pub type EfpiR = crate::FieldReader;
#[doc = "Field `EFPI` writer - 20:16\\]
Event FIFO Put Index"]
pub type EfpiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU71` reader - 23:21\\]
Reserved"]
pub type Nu71R = crate::FieldReader;
#[doc = "Field `NU71` writer - 23:21\\]
Reserved"]
pub type Nu71W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EFF` reader - 24:24\\]
Event FIFO Full"]
pub type EffR = crate::BitReader;
#[doc = "Field `EFF` writer - 24:24\\]
Event FIFO Full"]
pub type EffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFL` reader - 25:25\\]
Tx Event FIFO Element Lost"]
pub type TeflR = crate::BitReader;
#[doc = "Field `TEFL` writer - 25:25\\]
Tx Event FIFO Element Lost"]
pub type TeflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU72` reader - 31:26\\]
Reserved"]
pub type Nu72R = crate::FieldReader;
#[doc = "Field `NU72` writer - 31:26\\]
Reserved"]
pub type Nu72W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Event FIFO FIll Level"]
    #[inline(always)]
    pub fn effl(&self) -> EfflR {
        EfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Reserved"]
    #[inline(always)]
    pub fn nu69(&self) -> Nu69R {
        Nu69R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Event FIFO Get Index"]
    #[inline(always)]
    pub fn efgi(&self) -> EfgiR {
        EfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Reserved"]
    #[inline(always)]
    pub fn nu70(&self) -> Nu70R {
        Nu70R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Event FIFO Put Index"]
    #[inline(always)]
    pub fn efpi(&self) -> EfpiR {
        EfpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved"]
    #[inline(always)]
    pub fn nu71(&self) -> Nu71R {
        Nu71R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Event FIFO Full"]
    #[inline(always)]
    pub fn eff(&self) -> EffR {
        EffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    pub fn nu72(&self) -> Nu72R {
        Nu72R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Event FIFO FIll Level"]
    #[inline(always)]
    #[must_use]
    pub fn effl(&mut self) -> EfflW<TxefsSpec> {
        EfflW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu69(&mut self) -> Nu69W<TxefsSpec> {
        Nu69W::new(self, 6)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Event FIFO Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn efgi(&mut self) -> EfgiW<TxefsSpec> {
        EfgiW::new(self, 8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu70(&mut self) -> Nu70W<TxefsSpec> {
        Nu70W::new(self, 13)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Event FIFO Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn efpi(&mut self) -> EfpiW<TxefsSpec> {
        EfpiW::new(self, 16)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu71(&mut self) -> Nu71W<TxefsSpec> {
        Nu71W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Event FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn eff(&mut self) -> EffW<TxefsSpec> {
        EffW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Tx Event FIFO Element Lost"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TeflW<TxefsSpec> {
        TeflW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu72(&mut self) -> Nu72W<TxefsSpec> {
        Nu72W::new(self, 26)
    }
}
#[doc = "TXEFS\n\nYou can [`read`](crate::Reg::read) this register and get [`txefs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxefsSpec;
impl crate::RegisterSpec for TxefsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefs::R`](R) reader structure"]
impl crate::Readable for TxefsSpec {}
#[doc = "`write(|w| ..)` method takes [`txefs::W`](W) writer structure"]
impl crate::Writable for TxefsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TxefsSpec {
    const RESET_VALUE: u32 = 0;
}
